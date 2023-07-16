
type _obj = int;
type ty_param = int;
type node_id = int;
type _fn = int;

enum foo {
  item_obj(_obj, ~[ty_param], /* constructor id */node_id),
  item_res(_fn,
              /* dtor */
              node_id,
              /* dtor id */
              ~[ty_param]
           )
}
