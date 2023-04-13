use inquire::{formatter::DEFAULT_DATE_FROM_STR_FORMATTER, CustomType, DateFromStr};
use time::{macros::format_description, Date};

fn main() {
    let amount = CustomType::<DateFromStr>::new("When are you going to visit the office?")
        .with_placeholder("dd/mm/yyyy")
        .with_parser(&|i| {
            {
                match Date::parse(i, &format_description!(version = 2, "[day]/[month]/[year]")) {
                    Ok(date) => Ok(DateFromStr { date }),
                    Err(err) => Err(err),
                }
            }
            .map_err(|_| ())
        })
        .with_formatter(DEFAULT_DATE_FROM_STR_FORMATTER)
        .with_error_message("Please type a valid date.")
        .with_help_message("The necessary arrangements will be made")
        .prompt();

    match amount {
        Ok(_) => println!("Thanks! We will be expecting you."),
        Err(_) => println!("We could not process your reservation"),
    }
}
