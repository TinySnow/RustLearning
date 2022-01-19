mod hello_world;
mod primitives;
mod custom_types;
mod variable_bindings;
mod types;
mod conversion;
mod expressions;
mod flow_of_control;
mod functions;
mod modules;
mod use_crate;
mod use_cargo;
mod attributes;
mod generics;
mod scoping_rules;
mod traits;
mod macro_rules;
mod error_handling;


fn main() {
    hello_world::hello_world::main();
    println!("----------------------- hello_world::hello_world::main(); 结束 --------------------------");
    hello_world::comments::main();
    println!("----------------------- hello_world::comments::main(); 结束 --------------------------");
    hello_world::formatted_print::main();
    println!("----------------------- hello_world::formatted_print::main(); 结束 --------------------------");
    hello_world::formatted_print_debug::main();
    println!("----------------------- hello_world::formatted_print_debug::main(); 结束 --------------------------");
    hello_world::formatted_print_display::main();
    println!("----------------------- hello_world::formatted_print_display::main(); 结束 --------------------------");
    hello_world::formatted_print_display_list::main();
    println!("----------------------- hello_world::formatted_print_display_list::main(); 结束 --------------------------");
    hello_world::formatted_print_formatting::main();
    println!("----------------------- hello_world::formatted_print_formatting::main(); 结束 --------------------------");
    primitives::primitives::main();
    println!("----------------------- primitives::primitives::main(); 结束 --------------------------");
    primitives::literals_and_operators::main();
    println!("----------------------- primitives::literals_and_operators::main(); 结束 --------------------------");
    primitives::tuples::main();
    println!("----------------------- primitives::tuples::main(); 结束 --------------------------");
    primitives::arrays_and_slices::main();
    println!("----------------------- primitives::arrays_and_slices::main(); 结束 --------------------------");
    custom_types::custom_types::main();
    println!("----------------------- custom_types::custom_types::main(); 结束 --------------------------");
    custom_types::structures::main();
    println!("----------------------- custom_types::structures::main(); 结束 --------------------------");
    custom_types::enums::main();
    println!("----------------------- custom_types::enums::main(); 结束 --------------------------");
    custom_types::enums_use::main();
    println!("----------------------- custom_types::enums_use::main(); 结束 --------------------------");
    custom_types::enums_c_like::main();
    println!("----------------------- custom_types::enums_c_like::main(); 结束 --------------------------");
    custom_types::enums_linked_list::main();
    println!("----------------------- custom_types::enums_linked_list::main(); 结束 --------------------------");
    custom_types::constants::main();
    println!("----------------------- custom_types::constants::main(); 结束 --------------------------");
    variable_bindings::variable_bindings::main();
    println!("----------------------- variable_bindings::variable_bindings::main(); 结束 --------------------------");
    variable_bindings::mutability::main();
    println!("----------------------- variable_bindings::mutability::main(); 结束 --------------------------");
    variable_bindings::scope_and_shadowing::main();
    println!("----------------------- variable_bindings::scope_and_shadowing::main(); 结束 --------------------------");
    variable_bindings::declare_first::main();
    println!("----------------------- variable_bindings::declare_first::main(); 结束 --------------------------");
    variable_bindings::freezing::main();
    println!("----------------------- variable_bindings::freezing::main(); 结束 --------------------------");
    types::types::main();
    println!("----------------------- types::types::main(); 结束 --------------------------");
    types::casting::main();
    println!("----------------------- types::casting::main(); 结束 --------------------------");
    types::literals::main();
    println!("----------------------- types::literals::main(); 结束 --------------------------");
    types::inference::main();
    println!("----------------------- types::inference::main(); 结束 --------------------------");
    types::aliasing::main();
    println!("----------------------- types::aliasing::main(); 结束 --------------------------");
    conversion::conversion::main();
    println!("----------------------- conversion::conversion::main(); 结束 --------------------------");
    conversion::from_and_into::main();
    println!("----------------------- conversion::from_and_into::main(); 结束 --------------------------");
    conversion::tryfrom_and_tryinto::main();
    println!("----------------------- conversion::tryfrom_and_tryinto::main(); 结束 --------------------------");
    conversion::to_and_from_strings::main();
    println!("----------------------- conversion::to_and_from_strings::main(); 结束 --------------------------");
    expressions::expressions::main();
    println!("----------------------- expressions::expressions::main(); 结束 --------------------------");
    flow_of_control::flow_of_control::main();
    println!("----------------------- flow_of_control::flow_of_control::main(); 结束 --------------------------");
    flow_of_control::if_and_else::main();
    println!("----------------------- flow_of_control::if_and_else::main(); 结束 --------------------------");
    flow_of_control::loops::main();
    println!("----------------------- flow_of_control::loops::main(); 结束 --------------------------");
    flow_of_control::loops_nesting_and_labels::main();
    println!("----------------------- flow_of_control::loops_nesting_and_labels::main(); 结束 --------------------------");
    flow_of_control::loops_returning_from_loops::main();
    println!("----------------------- flow_of_control::loops_returning_from_loops::main(); 结束 --------------------------");
    flow_of_control::whiles::main();
    println!("----------------------- flow_of_control::whiles::main(); 结束 --------------------------");
    flow_of_control::for_and_range::main();
    println!("----------------------- flow_of_control::for_and_range::main(); 结束 --------------------------");
    flow_of_control::_match::main();
    println!("----------------------- flow_of_control::_match::main(); 结束 --------------------------");
    flow_of_control::_match_destructuring::main();
    println!("----------------------- flow_of_control::_match_destructuring::main(); 结束 --------------------------");
    flow_of_control::_match_destructuring_tuples::main();
    println!("----------------------- flow_of_control::_match_destructuring_tuples::main(); 结束 --------------------------");
    flow_of_control::_match_destructuring_enums::main();
    println!("----------------------- flow_of_control::_match_destructuring_enums::main(); 结束 --------------------------");
    flow_of_control::_match_destructuring_pointers_and_ref::main();
    println!("----------------------- flow_of_control::_match_destructuring_pointers_and_ref::main(); 结束 --------------------------");
    flow_of_control::_match_destructuring_structs::main();
    println!("----------------------- flow_of_control::_match_destructuring_structs::main(); 结束 --------------------------");
    flow_of_control::_match_guards::main();
    println!("----------------------- flow_of_control::_match_guards::main(); 结束 --------------------------");
    flow_of_control::_match_binding::main();
    println!("----------------------- flow_of_control::_match_binding::main(); 结束 --------------------------");
    flow_of_control::if_let::main();
    println!("----------------------- flow_of_control::if_let::main(); 结束 --------------------------");
    flow_of_control::while_let::main();
    println!("----------------------- flow_of_control::while_let::main(); 结束 --------------------------");
    functions::functions::main();
    println!("----------------------- functions::functions::main(); 结束 --------------------------");
    functions::methods::main();
    println!("----------------------- functions::methods::main(); 结束 --------------------------");
    functions::closures::main();
    println!("----------------------- functions::closures::main(); 结束 --------------------------");
    functions::closures_capturing::main();
    println!("----------------------- functions::closures_capturing::main(); 结束 --------------------------");
    functions::closures_as_input_parameters::main();
    println!("----------------------- functions::closures_as_input_parameters::main(); 结束 --------------------------");
    functions::closures_type_anonymity::main();
    println!("----------------------- functions::closures_type_anonymity::main(); 结束 --------------------------");
    functions::closures_input_functions::main();
    println!("----------------------- functions::closures_input_functions::main(); 结束 --------------------------");
    functions::closures_as_output_parameters::main();
    println!("----------------------- functions::closures_as_output_parameters::main(); 结束 --------------------------");
    functions::closures_examples_in_std::main();
    println!("----------------------- functions::closures_examples_in_std::main(); 结束 --------------------------");
    functions::closures_examples_in_std_iterator_any::main();
    println!("----------------------- functions::closures_examples_in_std_iterator_any::main(); 结束 --------------------------");
    functions::closures_examples_in_std_searching_through_iterators::main();
    println!("----------------------- functions::closures_examples_in_std_searching_through_iterators::main(); 结束 --------------------------");
    functions::higher_order_functions::main();
    println!("----------------------- functions::higher_order_functions::main(); 结束 --------------------------");
    functions::diverging_functions::main();
    println!("----------------------- functions::diverging_functions::main(); 结束 --------------------------");
    modules::modules::main();
    println!("----------------------- modules::modules::main(); 结束 --------------------------");
    modules::visibility::main();
    println!("----------------------- modules::visibility::main(); 结束 --------------------------");
    modules::structs_visibility::main();
    println!("----------------------- modules::structs_visibility::main(); 结束 --------------------------");
    modules::the_use_declaration::main();
    println!("----------------------- modules::the_use_declaration::main(); 结束 --------------------------");
    modules::super_and_self::main();
    println!("----------------------- modules::super_and_self::main(); 结束 --------------------------");
    modules::file_hierarchy::main();
    println!("----------------------- modules::file_hierarchy::main(); 结束 --------------------------");
    // use_crate::rary::main();
    // println!("----------------------- use_crate::rary::main(); 结束 --------------------------");
    // use_crate::executable::main();
    // println!("----------------------- use_crate::executable::main(); 结束 --------------------------");
    use_cargo::use_cargo::main();
    println!("----------------------- use_cargo::use_cargo::main(); 结束 --------------------------");
    attributes::attributes::main();
    println!("----------------------- attributes::attributes::main(); 结束 --------------------------");
    attributes::dead_code::main();
    println!("----------------------- attributes::dead_code::main(); 结束 --------------------------");
    attributes::use_cfg::main();
    println!("----------------------- attributes::use_cfg::main(); 结束 --------------------------");
    // attributes::use_cfg_custom::main();
    // println!("----------------------- attributes::use_cfg_custom::main(); 结束 --------------------------");
    generics::generics::main();
    println!("----------------------- generics::generics::main(); 结束 --------------------------");
    generics::functions::main();
    println!("----------------------- generics::functions::main(); 结束 --------------------------");
    generics::implementation::main();
    println!("----------------------- generics::implementation::main(); 结束 --------------------------");
    generics::_trait::main();
    println!("----------------------- generics::_trait::main(); 结束 --------------------------");
    generics::bounds::main();
    println!("----------------------- generics::bounds::main(); 结束 --------------------------");
    generics::bounds_empty_bounds::main();
    println!("----------------------- generics::bounds_empty_bounds::main(); 结束 --------------------------");
    generics::multiple_bounds::main();
    println!("----------------------- generics::multiple_bounds::main(); 结束 --------------------------");
    generics::where_clauses::main();
    println!("----------------------- generics::where_clauses::main(); 结束 --------------------------");
    generics::new_type_idioms::main();
    println!("----------------------- generics::new_type_idioms::main(); 结束 --------------------------");
    generics::associated_items::main();
    println!("----------------------- generics::associated_item::main(); 结束 --------------------------");
    generics::associated_items_the_problems::main();
    println!("----------------------- generics::associated_item_the_problems::main(); 结束 --------------------------");
    generics::associated_items_associated_types::main();
    println!("----------------------- generics::associated_items_associated_types::main(); 结束 --------------------------");
    generics::phantom_type_parameters::main();
    println!("----------------------- generics::phantom_type_parameters::main(); 结束 --------------------------");
    generics::phantom_type_parameters_unit_clarification::main();
    println!("----------------------- generics::phantom_type_parameters_unit_clarification::main(); 结束 --------------------------");
    scoping_rules::scoping_rules::main();
    println!("----------------------- scoping_rules::scoping_rules::main(); 结束 --------------------------");
    scoping_rules::raii::main();
    println!("----------------------- scoping_rules::raii::main(); 结束 --------------------------");
    scoping_rules::ownership_and_moves::main();
    println!("----------------------- scoping_rules::ownership_and_moves::main(); 结束 --------------------------");
    scoping_rules::ownership_and_moves_mutability::main();
    println!("----------------------- scoping_rules::ownership_and_moves_mutability::main(); 结束 --------------------------");
    scoping_rules::ownership_and_moves_partial_moves::main();
    println!("----------------------- scoping_rules::ownership_and_moves_partial_moves::main(); 结束 --------------------------");
    scoping_rules::borrowing::main();
    println!("----------------------- scoping_rules::borrowing::main(); 结束 --------------------------");
    scoping_rules::borrowing_mutability::main();
    println!("----------------------- scoping_rules::borrowing_mutability::main(); 结束 --------------------------");
    scoping_rules::borrowing_aliasing::main();
    println!("----------------------- scoping_rules::borrowing_aliasing::main(); 结束 --------------------------");
    scoping_rules::borrowing_the_ref_pattern::main();
    println!("----------------------- scoping_rules::borrowing_the_ref_pattern::main(); 结束 --------------------------");
    scoping_rules::lifetimes::main();
    println!("----------------------- scoping_rules::lifetimes::main(); 结束 --------------------------");
    scoping_rules::lifetimes_explicit_annotation::main();
    println!("----------------------- scoping_rules::lifetimes_explicit_annotation::main(); 结束 --------------------------");
    scoping_rules::lifetimes_functions::main();
    println!("----------------------- scoping_rules::lifetimes_functions::main(); 结束 --------------------------");
    scoping_rules::lifetimes_methods::main();
    println!("----------------------- scoping_rules::lifetimes_methods::main(); 结束 --------------------------");
    scoping_rules::lifetimes_structs::main();
    println!("----------------------- scoping_rules::lifetimes_structs::main(); 结束 --------------------------");
    scoping_rules::lifetimes_trait::main();
    println!("----------------------- scoping_rules::lifetimes_trait::main(); 结束 --------------------------");
    scoping_rules::lifetimes_bounds::main();
    println!("----------------------- scoping_rules::lifetimes_bounds::main(); 结束 --------------------------");
    scoping_rules::lifetimes_coercion::main();
    println!("----------------------- scoping_rules::lifetimes_coercion::main(); 结束 --------------------------");
    scoping_rules::lifetimes_static::main();
    println!("----------------------- scoping_rules::lifetimes_static::main(); 结束 --------------------------");
    scoping_rules::lifetimes_elision::main();
    println!("----------------------- scoping_rules::lifetimes_elision::main(); 结束 --------------------------");
    traits::traits::main();
    println!("----------------------- traits::traits::main(); 结束 --------------------------");
    traits::derive::main();
    println!("----------------------- traits::derive::main(); 结束 --------------------------");
    traits::returning_traits_with_dyn::main();
    println!("----------------------- traits::returning_traits_with_dyn::main(); 结束 --------------------------");
    traits::operator_overloading::main();
    println!("----------------------- traits::operator_overloading::main(); 结束 --------------------------");
    traits::drop::main();
    println!("----------------------- traits::drop::main(); 结束 --------------------------");
    traits::iterators::main();
    println!("----------------------- traits::iterators::main(); 结束 --------------------------");
    traits::impl_trait::main();
    println!("----------------------- traits::iterators::main(); 结束 --------------------------");
    traits::clone::main();
    println!("----------------------- traits::clone::main(); 结束 --------------------------");
    traits::supertraits::main();
    println!("----------------------- traits::supertraits::main(); 结束 --------------------------");
    traits::disambiguating_overlapping_traits::main();
    println!("----------------------- traits::disambiguating_overlapping_traits::main(); 结束 --------------------------");
    macro_rules::macro_rules::main();
    println!("----------------------- macro_rules::macro_rules::main(); 结束 --------------------------");
    macro_rules::syntax::main();
    println!("----------------------- macro_rules::syntax::main(); 结束 --------------------------");
    macro_rules::syntax_designators::main();
    println!("----------------------- macro_rules::syntax_designators::main(); 结束 --------------------------");
    macro_rules::syntax_overload::main();
    println!("----------------------- macro_rules::syntax_overload::main(); 结束 --------------------------");
    macro_rules::syntax_repeat::main();
    println!("----------------------- macro_rules::syntax_repeat::main(); 结束 --------------------------");
    // macro_rules::dry::main();
    // println!("----------------------- macro_rules::dry::main(); 结束 --------------------------");
    macro_rules::dsl::main();
    println!("----------------------- macro_rules::dsl::main(); 结束 --------------------------");
    macro_rules::variadic_interfaces::main();
    println!("----------------------- macro_rules::variadic_interfaces::main(); 结束 --------------------------");
    error_handling::error_handling::main();
    println!("----------------------- error_handling::error_handling::main(); 结束 --------------------------");
    error_handling::panic::main();
    println!("----------------------- error_handling::panic::main(); 结束 --------------------------");
    error_handling::option_and_unwrap::main();
    println!("----------------------- error_handling::option_and_unwrap::main(); 结束 --------------------------");
    error_handling::option_and_unwrap_unpacking_options_with_question_mark::main();
    println!("----------------------- error_handling::option_and_unwrap_unpacking_options_with_question_mark::main(); 结束 --------------------------");
    error_handling::option_and_unwrap_map_combinator::main();
    println!("----------------------- error_handling::option_and_unwrap_map_combinator::main(); 结束 --------------------------");


}
