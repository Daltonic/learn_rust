mod module_01_data_types_examples;
mod module_02_control_flow_examples;
mod module_03_functions_closure_examples;
mod module_04_data_structures_examples;
mod module_05_ownership_examples;
mod module_06_struct_examples;
mod module_07_enum_examples;
mod module_08_option_examples;
mod module_09_result_examples;
mod module_10_option_result_combinations_examples;
mod module_11_traits_examples;
mod module_12_concurrency_examples;

#[tokio::main]
async fn main() {
    module_01_data_types_examples::demo();
    module_02_control_flow_examples::demo();
    module_03_functions_closure_examples::demo();
    module_04_data_structures_examples::demo();
    module_05_ownership_examples::demo();
    module_06_struct_examples::demo();
    module_07_enum_examples::demo();
    module_08_option_examples::demo();
    module_09_result_examples::demo();
    module_10_option_result_combinations_examples::demo();
    module_11_traits_examples::demo();
    module_12_concurrency_examples::demo().await;

    print!("\n\n\n\n\n\n\n\n\n")
}
