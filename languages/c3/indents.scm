(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
(_ "[" "]" @end) @indent

(ct_if_stmt "$endif" @end) @indent
(ct_for_stmt "$endfor" @end) @indent
(ct_foreach_stmt "$endforeach" @end) @indent
(ct_switch_stmt "$endswitch" @end) @indent

[
  (case_stmt)
  (default_stmt)
  (ct_case_stmt)
] @indent
