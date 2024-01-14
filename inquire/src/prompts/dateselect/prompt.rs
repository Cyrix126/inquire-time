use std::{
    cmp::{max, min, Ordering},
    ops::Add,
};

use crate::{
    date_utils::get_current_date,
    error::InquireResult,
    formatter::DateFormatter,
    prompts::prompt::{ActionResult, Prompt},
    ui::date::DateSelectBackend,
    validator::{DateValidator, ErrorMessage, Validation},
    DateSelect, InquireError,
};
use time::util::days_in_year_month;
use time::{Date, Duration, Month};

use super::{action::DateSelectPromptAction, config::DateSelectConfig};

pub struct DateSelectPrompt<'a> {
    message: &'a str,
    config: DateSelectConfig,
    current_date: Date,
    help_message: Option<&'a str>,
    formatter: DateFormatter<'a>,
    validators: Vec<Box<dyn DateValidator>>,
    error: Option<ErrorMessage>,
}

impl<'a> DateSelectPrompt<'a> {
    pub fn new(so: DateSelect<'a>) -> InquireResult<Self> {
        if let Some(min_date) = so.min_date {
            if min_date > so.starting_date {
                return Err(InquireError::InvalidConfiguration(
                    "Min date can not be greater than starting date".into(),
                ));
            }
        }
        if let Some(max_date) = so.max_date {
            if max_date < so.starting_date {
                return Err(InquireError::InvalidConfiguration(
                    "Max date can not be smaller than starting date".into(),
                ));
            }
        }

        Ok(Self {
            message: so.message,
            current_date: so.starting_date,
            config: (&so).into(),
            help_message: so.help_message,
            formatter: so.formatter,
            validators: so.validators,
            error: None,
        })
    }

    fn shift_date(&mut self, duration: Duration) -> ActionResult {
        self.update_date(self.current_date.add(duration))
    }

    fn shift_months(&mut self, qty: i32) -> ActionResult {
        let month = self.current_date.month();
        let mut new_month = month;
        let mut new_day = self.current_date.day();
        let new_date = match qty.cmp(&0) {
            Ordering::Greater | Ordering::Equal => {
                let mut year_to_add = 0;
                for _ in 0..qty {
                    new_month = month.next();
                    if new_month == Month::January {
                        year_to_add += 1;
                    }
                }
                let new_year = self.current_date.year() + year_to_add;
                let last_day = days_in_year_month(new_year, new_month);
                if self.current_date.day() > last_day {
                    new_day = last_day;
                }
                Date::from_calendar_date(new_year, new_month, new_day).unwrap_or(self.current_date)
            }

            Ordering::Less => {
                let qty = qty.abs();
                let mut year_to_remove = 0;
                for _ in 0..qty {
                    new_month = month.previous();
                    if new_month == Month::December {
                        year_to_remove += 1;
                    }
                }
                let new_year = self.current_date.year() - year_to_remove;
                let last_day = days_in_year_month(new_year, new_month);
                if self.current_date.day() > last_day {
                    new_day = last_day;
                }
                Date::from_calendar_date(new_year, new_month, new_day).unwrap_or(self.current_date)
            }
        };
        self.update_date(new_date)
    }

    fn update_date(&mut self, new_date: Date) -> ActionResult {
        if self.current_date == new_date {
            return ActionResult::Clean;
        }

        self.current_date = new_date;
        if let Some(min_date) = self.config.min_date {
            self.current_date = max(self.current_date, min_date);
        }
        if let Some(max_date) = self.config.max_date {
            self.current_date = min(self.current_date, max_date);
        }

        ActionResult::NeedsRedraw
    }

    fn validate_current_answer(&self) -> InquireResult<Validation> {
        for validator in &self.validators {
            match validator.validate(self.cur_answer()) {
                Ok(Validation::Valid) => {}
                Ok(Validation::Invalid(msg)) => return Ok(Validation::Invalid(msg)),
                Err(err) => return Err(InquireError::Custom(err)),
            }
        }

        Ok(Validation::Valid)
    }

    fn cur_answer(&self) -> Date {
        self.current_date
    }
}

impl<'a, B> Prompt<B> for DateSelectPrompt<'a>
where
    B: DateSelectBackend,
{
    type Config = DateSelectConfig;
    type InnerAction = DateSelectPromptAction;
    type Output = Date;

    fn message(&self) -> &str {
        self.message
    }

    fn format_answer(&self, answer: &Date) -> String {
        (self.formatter)(*answer)
    }

    fn config(&self) -> &DateSelectConfig {
        &self.config
    }

    fn submit(&mut self) -> InquireResult<Option<Date>> {
        let answer = match self.validate_current_answer()? {
            Validation::Valid => Some(self.cur_answer()),
            Validation::Invalid(msg) => {
                self.error = Some(msg);
                None
            }
        };

        Ok(answer)
    }

    fn handle(&mut self, action: DateSelectPromptAction) -> InquireResult<ActionResult> {
        let result = match action {
            DateSelectPromptAction::GoToPrevWeek => self.shift_date(Duration::weeks(-1)),
            DateSelectPromptAction::GoToNextWeek => self.shift_date(Duration::weeks(1)),
            DateSelectPromptAction::GoToPrevDay => self.shift_date(Duration::days(-1)),
            DateSelectPromptAction::GoToNextDay => self.shift_date(Duration::days(1)),
            DateSelectPromptAction::GoToPrevYear => self.shift_months(-12),
            DateSelectPromptAction::GoToNextYear => self.shift_months(12),
            DateSelectPromptAction::GoToPrevMonth => self.shift_months(-1),
            DateSelectPromptAction::GoToNextMonth => self.shift_months(1),
        };

        Ok(result)
    }

    fn render(&self, backend: &mut B) -> InquireResult<()> {
        let prompt = &self.message;

        if let Some(err) = &self.error {
            backend.render_error_message(err)?;
        }

        backend.render_calendar_prompt(prompt)?;

        backend.render_calendar(
            self.current_date.month(),
            self.current_date.year(),
            self.config.week_start,
            get_current_date(),
            self.current_date,
            self.config.min_date,
            self.config.max_date,
        )?;

        if let Some(help_message) = self.help_message {
            backend.render_help_message(help_message)?;
        }

        Ok(())
    }
}
