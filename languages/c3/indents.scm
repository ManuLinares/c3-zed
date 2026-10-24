[
  (compound_stmt)
  (initializer_list)
  (implies_body)
  (struct_body)
  (bitstruct_body)
  (enum_body)
  (interface_body)
  (switch_body)
  (ct_if_stmt)
  (ct_for_stmt)
  (ct_foreach_stmt)
  (ct_switch_stmt)
] @indent

[
  (case_stmt)
  (default_stmt)
  (ct_case_stmt)
] @indent

(expr_stmt
  ";" @end) @indent

(declaration_stmt
  ";" @end) @indent

(return_stmt
  ";" @end) @indent

(global_declaration
  ";" @end) @indent

(faultdef_declaration
  ";" @end) @indent

(macro_func_body
  ";" @end)

[
  ")"
  "}"
  "$endfor"
  "$endforeach"
  "$endswitch"
  "$endif"
] @outdent @end

"$else" @outdent

[
  (func_param_list)
  (macro_param_list)
  (enum_param_list)
  (attribute_param_list)
  (call_arg_list)
  (paren_cond)
  (for_cond)
  (foreach_cond)
  (paren_expr)
] @indent

[
  (block_comment)
  (doc_comment)
  (raw_string_literal)
  (bytes_literal)
] @_indent.auto

(string_literal) @_indent.ignore
