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

pub fn get_tasks_table(width: u8) -> ConsoleTableBuilder<Task> {
    let id = TaskColumn::new("ID", |x| x.id.to_string()).set_data_alignment(Alignment::Right);
    let description = TaskColumn::new("Description", |x| x.description.to_string());
    let scope = TaskColumn::new("Scope", |x| match x.scope.as_ref() {
        Some(v) => v.as_ref().to_string(),
        None => "None".to_string(),
    });
    let created_at = TaskColumn::new("Created at", |x| {
        x.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
    });
    let completed_at = TaskColumn::new("Completed", |x| {
        x.completed_at.map_or(" ", |_| "x").to_string()
    });

    ConsoleTableBuilder::<Task>::new(width)
        .add_column(id, 1)
        .add_column(description, 8)
        .add_column(scope, 3)
        .add_column(created_at, 4)
        .add_column(completed_at, 2)
}

type TaskColumn = Column<Task>;

type ColumnValueGetter<T> = fn(input: &T) -> String;

pub struct Column<T> {
    pub name: String,
    pub column_alignment: Alignment,
    pub data_alignment: Alignment,
    get_value: ColumnValueGetter<T>,
}

impl<T> Column<T> {
    pub fn new(name: &str, getter: ColumnValueGetter<T>) -> Self {
        Self {
            name: name.to_string(),
            column_alignment: Alignment::Center,
            data_alignment: Alignment::Center,
            get_value: getter,
        }
    }

    pub fn set_column_alignment(mut self, alignment: Alignment) -> Self {
        self.column_alignment = alignment;
        self
    }

    pub fn set_data_alignment(mut self, alignment: Alignment) -> Self {
        self.data_alignment = alignment;
        self
    }
}

pub struct ConsoleTable<T> {
    width: u8,
    pub vertical_separator: char,
    pub horizontal_separator: char,
    pub cross_separator: char,
    columns: Vec<(Column<T>, u8)>,
}

pub struct ConsoleTableBuilder<T> {
    pub width: u8,
    columns: Vec<(Column<T>, u8)>,
    vertical_separator: char,
    horizontal_separator: char,
    cross_separator: char,
}

impl<T> ConsoleTableBuilder<T> {
    pub fn new(width: u8) -> Self {
        Self {
            width,
            columns: vec![],
            vertical_separator: '|',
            horizontal_separator: '-',
            cross_separator: '+',
        }
    }
    pub fn add_column(mut self, column: Column<T>, weight: u8) -> Self {
        self.columns.push((column, weight));
        self
    }

    pub fn build(self) -> Result<ConsoleTable<T>, ConsoleTableError> {
        if self.columns.len() >= 255 {
            return Err(ConsoleTableError::LengthExceeded);
        }

        if self.columns.is_empty() {
            return Err(ConsoleTableError::NoColumns);
        }
        if self.width < self.get_min_width() {
            return Err(ConsoleTableError::NotEnoughSpaceToPrint);
        }

        Ok(ConsoleTable {
            width: self.width,
            columns: self.columns,
            vertical_separator: self.vertical_separator,
            horizontal_separator: self.horizontal_separator,
            cross_separator: self.cross_separator,
        })
    }

    fn get_min_width(&self) -> u8 {
        let column_len: u8 = self.columns.len().try_into().unwrap();
        let spacing_witdh: u8 = column_len + 1;
        let columns_width: u8 = self.columns.iter().map(|c| c.1).sum();
        spacing_witdh + columns_width
    }
}

impl<T> ConsoleTable<T> {
    pub fn print<I>(&self, data: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.print_separator();
        self.print_header();
        self.print_separator();
        self.print_data(data);
        self.print_separator();
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
                get_formatted_cell(x.0.name.as_ref(), width, x.0.column_alignment)
            })
            .join(&self.vertical_separator.to_string());
        let column_header_text = add_value_to_start_and_end_of_string(
            column_header_text,
            &self.vertical_separator.to_string(),
        );
        println!("{}", column_header_text);
    }

    fn print_data<I>(&self, data: I)
    where
        I: IntoIterator<Item = T>,
    {
        let unit_width = self.get_unit_width();
        for row in data.into_iter() {
            let data_text: String = self
                .columns
                .iter()
                .map(|x| {
                    let width: usize = (x.1 * unit_width).into();
                    let value = (x.0.get_value)(&row);
                    get_formatted_cell(&value, width, x.0.data_alignment)
                })
                .join(&self.vertical_separator.to_string());
            let column_header_text = add_value_to_start_and_end_of_string(
                data_text,
                &self.vertical_separator.to_string(),
            );
            println!("{}", column_header_text);
        }
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
