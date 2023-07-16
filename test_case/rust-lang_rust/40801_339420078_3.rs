
let x = match v {
  Case1 => <?T as Deserialize>::deserialize(),
  Case2 => Err::<?U, E>(e) // `?U` gets fallback here
};
