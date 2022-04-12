
pub mod fy{
    pub fn fy_handle(input: String) -> String {
        let handled = "Handled:";
        format!("{}{}", handled, input).to_string()
    }
}
