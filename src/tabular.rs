use std::fmt::Alignment;

use itertools::Itertools;

use crate::domain::Task;

fn format_string_to_constraint(text: &str, max_len: usize) -> &str {
    let text_len = text.len();
    if text_len <= max_len {
        return text;
    }
    &text[0..max_len]
}

pub fn get_tasks_table(width: u8) -> ConsoleTable<Task> {
    let mut table = ConsoleTable::<Task>::new(width);
    table
        .add_column(TaskColumn::new("ID", |x| x.id.to_string()), 2)
        .unwrap();
    table
        .add_column(
            TaskColumn::new("Description", |x| x.description.to_string()),
            8,
        )
        .unwrap();
    table
        .add_column(
            TaskColumn::new("Scope", |x| {
                x.scope
                    .as_ref()
                    .map_or("None".to_string(), |s| s.as_ref().to_string())
            }),
            3,
        )
        .unwrap();

    table
        .add_column(
            TaskColumn::new("Created at", |x| {
                x.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
            }),
            6,
        )
        .unwrap();
    table
        .add_column(
            TaskColumn::new("Completed", |x| {
                x.completed_at.map_or(" ", |_| "x").to_string()
            }),
            4,
        )
        .unwrap();
    table
}

type TaskColumn = Column<Task>;

type ColumnValueGetter<T> = fn(input: &T) -> String;

pub struct Column<T> {
    pub name: String,
    pub text_alignment: Alignment,
    get_value: ColumnValueGetter<T>,
}

impl<T> Column<T> {
    pub fn new(name: &str, getter: ColumnValueGetter<T>) -> Self {
        Self {
            name: name.to_string(),
            text_alignment: Alignment::Center,
            get_value: getter,
        }
    }
}

pub struct ConsoleTable<T> {
    width: u8,
    pub vertical_separator: char,
    pub horizontal_separator: char,
    pub cross_separator: char,
    columns: Vec<(Column<T>, u8)>,
    data: Vec<T>,
}

impl<T> ConsoleTable<T> {
    pub fn new(width: u8) -> Self {
        Self {
            width,
            vertical_separator: '|',
            horizontal_separator: '-',
            cross_separator: '+',
            columns: vec![],
            data: vec![],
        }
    }

    pub fn add_column(
        &mut self,
        column: Column<T>,
        weight: u8,
    ) -> Result<&Self, ConsoleTableError> {
        if self.columns.len() >= 255 {
            return Err(ConsoleTableError::LengthExceeded);
        }
        self.columns.push((column, weight));
        Ok(self)
    }

    pub fn set_data(&mut self, data: Vec<T>) -> &Self {
        self.data = data;
        self
    }

    pub fn print(&self) -> Result<(), ConsoleTableError> {
        if self.columns.is_empty() {
            return Err(ConsoleTableError::NoColumns);
        }
        if self.width < self.get_min_width() {
            return Err(ConsoleTableError::NotEnoughSpaceToPrint);
        }

        self.print_separator();
        self.print_header();
        self.print_separator();
        self.print_data();
        self.print_separator();

        Ok(())
    }

    fn print_separator(&self) {
        let unit_width = self.get_unit_width();
        let separator_text: String = self
            .columns
            .iter()
            .map(|x| {
                let repeat_count: usize = (x.1 * unit_width).into();
                self.horizontal_separator.to_string().repeat(repeat_count)
            })
            .join(&self.cross_separator.to_string());
        let separator_text: String =
            add_value_to_start_and_end_of_string(separator_text, &self.cross_separator.to_string());
        println!("{}", separator_text);
    }

    fn print_header(&self) {
        let unit_width = self.get_unit_width();
        let column_header_text: String = self
            .columns
            .iter()
            .map(|x| {
                let width: usize = (x.1 * unit_width).into();
                get_formatted_cell(x.0.name.as_ref(), width, x.0.text_alignment)
            })
            .join(&self.vertical_separator.to_string());
        let column_header_text = add_value_to_start_and_end_of_string(
            column_header_text,
            &self.vertical_separator.to_string(),
        );
        println!("{}", column_header_text);
    }

    fn print_data(&self) {
        let unit_width = self.get_unit_width();
        for row in self.data.iter() {
            let data_text: String = self
                .columns
                .iter()
                .map(|x| {
                    let width: usize = (x.1 * unit_width).into();
                    let value = (x.0.get_value)(row);
                    get_formatted_cell(&value, width, x.0.text_alignment)
                })
                .join(&self.vertical_separator.to_string());
            let column_header_text = add_value_to_start_and_end_of_string(
                data_text,
                &self.vertical_separator.to_string(),
            );
            println!("{}", column_header_text);
        }
    }

    fn get_min_width(&self) -> u8 {
        let column_len: u8 = self.columns.len().try_into().unwrap();
        let spacing_witdh: u8 = column_len + 1;
        let columns_width: u8 = self.columns.iter().map(|c| c.1).sum();
        spacing_witdh + columns_width
    }

    fn get_unit_width(&self) -> u8 {
        let column_len: u8 = self.columns.len().try_into().unwrap();
        let spacing_witdh: u8 = column_len + 1;

        let total_units: u8 = self.columns.iter().map(|c| c.1).sum();
        (self.width - spacing_witdh) / total_units
    }
}

#[derive(Debug)]
pub enum ConsoleTableError {
    LengthExceeded,
    NotEnoughSpaceToPrint,
    NoColumns,
}

fn add_value_to_start_and_end_of_string(mut text: String, value_to_add: &str) -> String {
    text.insert_str(0, value_to_add);
    text.push_str(value_to_add);
    text
}

fn get_formatted_cell(value: &str, width: usize, alignment: Alignment) -> String {
    match alignment {
        Alignment::Left => format!(
            "{:<width$}",
            format_string_to_constraint(value, width),
            width = width
        ),
        Alignment::Right => format!(
            "{:>width$}",
            format_string_to_constraint(value, width),
            width = width
        ),
        Alignment::Center => format!(
            "{:^width$}",
            format_string_to_constraint(value, width),
            width = width
        ),
    }
}
