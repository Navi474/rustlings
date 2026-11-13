// Символи (Characters) (`char`)

fn main() {
    // Зверніть увагу на _одинарні_ лапки, вони відрізняються від подвійних лапок,
    // які ви бачили раніше.
    let my_first_initial = 'Є';
    if my_first_initial.is_alphabetic() {
        println!("Алфавітний!");
    } else if my_first_initial.is_numeric() {
        println!("Числовий!");
    } else {
        println!("Ні алфавітний, ні числовий!");
    }

    // TODO: Аналогічно до прикладу вище, оголосіть змінну з назвою `your_character`
    // нижче з вашим улюбленим символом.
    // Спробуйте літеру, спробуйте цифру (в одинарних лапках), спробуйте спеціальний символ,
    // спробуйте символ з іншої мови, спробуйте емодзі 😉
    // let your_character = '';
    let your_character = 'a';

    if your_character.is_alphabetic() {
        println!("Алфавітний!");
    } else if your_character.is_numeric() {
        println!("Числовий!");
    } else {
        println!("Ні алфавітний, ні числовий!");
    }
}
