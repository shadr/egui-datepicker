/// A translation for strings used by egui-datepicker
pub struct DateTranslation {
    /// The full-lenght twelve month of a year (starting at January)
    pub month: [&'static str; 12],
    /// A shortened version of the 7 days of the week (starting at Sunday)
    pub weekday_short: [&'static str; 7],
    /// Translation of the "today" button, that set the date to the current date when clicked
    pub today: &'static str,
}

pub const TRANSLATION_ENGLISH: DateTranslation = DateTranslation {
    month: [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ],
    weekday_short: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
    today: "Today",
};

pub const TRANSLATION_FRENCH: DateTranslation = DateTranslation {
    month: [
        "janvier",
        "février",
        "mars",
        "avril",
        "mai",
        "juin",
        "juillet",
        "août",
        "septembre",
        "octobre",
        "novembre",
        "décembre",
    ],
    weekday_short: ["dim.", "lun.", "mar.", "mer.", "jeu.", "ven.", "sam."],
    today: "Aujourd'hui",
};
