use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, one_of, space0, space1},
    combinator::{map_res, opt},
    multi::{many0, separated_list},
    number::complete::double,
};
pub use nom::{
    error::{convert_error, ParseError, VerboseError},
    Err, IResult,
};
#[derive(PartialEq, Clone, Debug)]
pub enum FzStmt {
    Predicate(PredicateItem),
    Parameter(ParDeclItem),
    Variable(VarDeclItem),
    Constraint(ConstraintItem),
    SolveItem(SolveItem),
}
pub fn fz_statement<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, FzStmt, E> {
    let (input, res) = alt((
        fz_predicate,
        fz_parameter,
        fz_variable,
        fz_constraint,
        fz_solve_item,
    ))(input)?;
    Ok((input, res))
}
fn fz_predicate<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, FzStmt, E> {
    let (input, item) = predicate_item(input)?;
    Ok((input, FzStmt::Predicate(item)))
}
fn fz_parameter<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, FzStmt, E> {
    let (input, item) = par_decl_item(input)?;
    Ok((input, FzStmt::Parameter(item)))
}
fn fz_variable<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, FzStmt, E> {
    let (input, item) = var_decl_item(input)?;
    Ok((input, FzStmt::Variable(item)))
}
fn fz_constraint<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, FzStmt, E> {
    let (input, item) = constraint_item(input)?;
    Ok((input, FzStmt::Constraint(item)))
}
fn fz_solve_item<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, FzStmt, E> {
    let (input, item) = solve_item(input)?;
    Ok((input, FzStmt::SolveItem(item)))
}
#[derive(PartialEq, Clone, Debug)]
pub struct Model {
    pub predicate_items: Vec<PredicateItem>,
    pub par_decl_items: Vec<ParDeclItem>,
    pub var_decl_items: Vec<VarDeclItem>,
    pub constraint_items: Vec<ConstraintItem>,
    pub solve_item: SolveItem,
}
pub fn model<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Model, E> {
    let (input, predicate_items) = many0(predicate_item_ln)(input)?;
    let (input, par_decl_items) = many0(par_decl_item_ln)(input)?;
    let (input, var_decl_items) = many0(var_decl_item_ln)(input)?;
    let (input, constraint_items) = many0(constraint_item_ln)(input)?;
    let (input, solve_item) = solve_item_ln(input)?;
    Ok((
        input,
        Model {
            predicate_items,
            par_decl_items,
            var_decl_items,
            constraint_items,
            solve_item,
        },
    ))
}
#[derive(PartialEq, Clone, Debug)]
pub struct PredicateItem {
    pub id: String,
    pub parameters: Vec<(PredParType, String)>,
}
fn predicate_item_ln<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PredicateItem, E> {
    let (input, item) = predicate_item(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('\n')(input)?;
    Ok((input, item))
}
pub fn predicate_item<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PredicateItem, E> {
    let (input, _) = tag("predicate")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = identifier(input)?;
    let (input, _) = char('(')(input)?;
    let (input, parameters) = separated_list(char(','), pred_par_type_ident_pair)(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, PredicateItem { id, parameters }))
}
fn pred_par_type_ident_pair<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (PredParType, String), E> {
    let (input, pred_par_type) = pred_par_type(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space0(input)?;
    let (input, ident) = identifier(input)?;
    Ok((input, (pred_par_type, ident)))
}
#[derive(PartialEq, Clone, Debug)]
pub enum BasicParType {
    Bool,
    Int,
    Float,
    SetOfInt,
}
fn basic_par_type<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BasicParType, E> {
    let (input, bpt) = alt((bpt_bool, bpt_int, bpt_float, bpt_set_of_int))(input)?;
    Ok((input, bpt))
}

fn bpt_bool<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BasicParType, E> {
    let (input, _tag) = tag("bool")(input)?;
    Ok((input, BasicParType::Bool))
}
fn bpt_int<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BasicParType, E> {
    let (input, _tag) = tag("int")(input)?;
    Ok((input, BasicParType::Int))
}
fn bpt_float<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BasicParType, E> {
    let (input, _tag) = tag("float")(input)?;
    Ok((input, BasicParType::Float))
}
// "var" "set" "of" "int"
// Moved this be a basic-var-type basic-par-type
fn bpt_set_of_int<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BasicParType, E> {
    let (input, _tag) = tag("set")(input)?;
    let (input, _) = space1(input)?;
    let (input, _tag) = tag("of")(input)?;
    let (input, _) = space1(input)?;
    let (input, _tag) = tag("int")(input)?;
    Ok((input, BasicParType::SetOfInt))
}
#[derive(PartialEq, Clone, Debug)]
pub enum ParType {
    Basic(BasicParType),
    Array {
        ix: IndexSet,
        par_type: BasicParType,
    },
}
#[test]
fn test_par_type() {
    use nom::error::VerboseError;
    assert_eq!(
        par_type::<VerboseError<&str>>("array [1..3] of float"),
        Ok((
            "",
            ParType::Array {
                ix: IndexSet(3),
                par_type: BasicParType::Float
            }
        ))
    );
}
fn par_type<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ParType, E> {
    let (input, par_type) = alt((pt_basic_par_type, array_par_type))(input)?;
    Ok((input, par_type))
}
fn pt_basic_par_type<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ParType, E> {
    let (input, pt) = basic_par_type(input)?;
    Ok((input, ParType::Basic(pt)))
}
fn array_par_type<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ParType, E> {
    let (input, _) = tag("array")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = char('[')(input)?;
    let (input, _) = space0(input)?;
    let (input, int) = index_set(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(']')(input)?;
    let (input, _) = space1(input)?;
    let (input, _tag) = tag("of")(input)?;
    let (input, _) = space1(input)?;
    let (input, par_type) = basic_par_type(input)?;
    Ok((
        input,
        ParType::Array {
            ix: IndexSet(int),
            par_type,
        },
    ))
}
#[derive(PartialEq, Clone, Debug)]
pub enum BasicVarType {
    Bool,
    Int,
    Float,
    Domain(Domain),
    SetOfInt, // added var_set_of_int from basic_pred_par_type
}
fn basic_var_type<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BasicVarType, E> {
    let (input, _) = space0(input)?;
    let (input, _tag) = tag("var")(input)?;
    let (input, _) = space1(input)?;
    let (input, bvt) = alt((bvt_basic_par_type, bvt_domain))(input)?;
    Ok((input, bvt))
}
fn bvt_basic_par_type<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicVarType, E> {
    let (input, bpt) = basic_par_type(input)?;
    match bpt {
        BasicParType::Bool => Ok((input, BasicVarType::Bool)),
        BasicParType::Int => Ok((input, BasicVarType::Int)),
        BasicParType::Float => Ok((input, BasicVarType::Float)),
        BasicParType::SetOfInt => Ok((input, BasicVarType::SetOfInt)),
    }
}
fn bvt_domain<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BasicVarType, E> {
    let (input, domain) = domain(input)?;
    Ok((input, BasicVarType::Domain(domain)))
}
// introduced by me used in basic-var-type and basic-pred-param-type
#[derive(PartialEq, Clone, Debug)]
pub enum Domain {
    IntRange(i128, i128),
    FloatRange(f64, f64),
    SetIntNonEmpty(Vec<i128>),
    SetIntRange(i128, i128),
    SetInt(Vec<i128>), // possibly empty
}
fn domain<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Domain, E> {
    let (input, domain) = alt((
        int_range,
        float_range,
        set_of_int_range,
        set_of_ints,
        set_of_ints_non_empty,
    ))(input)?;
    Ok((input, domain))
}
fn int_range<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Domain, E> {
    let (input, lb) = int_literal(input)?;
    let (input, _) = space0(input)?;
    let (input, _tag) = tag("..")(input)?;
    let (input, _) = space0(input)?;
    let (input, ub) = int_literal(input)?;
    Ok((input, Domain::IntRange(lb, ub)))
}
fn float_range<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Domain, E> {
    let (input, lb) = float_literal(input)?;
    let (input, _) = space0(input)?;
    let (input, _tag) = tag("..")(input)?;
    let (input, _) = space0(input)?;
    let (input, ub) = float_literal(input)?;
    Ok((input, Domain::FloatRange(lb, ub)))
}
// "set" "of" <int_literal> ".." <int_literal>
fn set_of_int_range<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Domain, E> {
    let (input, _tag) = tag("set")(input)?;
    let (input, _) = space1(input)?;
    let (input, _tag) = tag("of")(input)?;
    let (input, _) = space1(input)?;
    let (input, lb) = int_literal(input)?;
    let (input, _) = space0(input)?;
    let (input, _tag) = tag("..")(input)?;
    let (input, _) = space0(input)?;
    let (input, ub) = int_literal(input)?;
    Ok((input, Domain::SetIntRange(lb, ub)))
}
// "set" "of" "{" [ <int-literal> "," ... ] "}"
fn set_of_ints<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Domain, E> {
    let (input, _tag) = tag("set of {")(input)?;
    let (input, _) = space0(input)?;
    let (input, v) = separated_list(char(','), int_literal)(input)?;
    let (input, _) = space0(input)?;
    let (input, _tag) = tag("}")(input)?;
    Ok((input, Domain::SetInt(v)))
}
// "{" <int-literal> "," ... "}"
fn set_of_ints_non_empty<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Domain, E> {
    let (input, _) = char('{')(input)?;
    let (input, _) = space0(input)?;
    let (input, v) = separated_list(char(','), int_literal)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('}')(input)?;
    Ok((input, Domain::SetIntNonEmpty(v)))
}
#[derive(PartialEq, Clone, Debug)]
pub struct IndexSet(pub i128);
fn index_set<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i128, E> {
    let (input, _) = char('1')(input)?;
    let (input, _tag) = tag("..")(input)?;
    let (input, int) = int_literal(input)?;
    Ok((input, int))
}
#[derive(PartialEq, Clone, Debug)]
pub enum BasicPredParType {
    BasicParType(BasicParType),
    BasicVarType(BasicVarType),
    Domain(Domain),
}
fn basic_pred_par_type<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicPredParType, E> {
    let (input, bppt) = alt((bppt_basic_par_type, bppt_basic_var_type, bppt_domain))(input)?;
    Ok((input, bppt))
}
fn bppt_basic_par_type<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicPredParType, E> {
    let (input, bpt) = basic_par_type(input)?;
    Ok((input, BasicPredParType::BasicParType(bpt)))
}
fn bppt_basic_var_type<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicPredParType, E> {
    let (input, bvt) = basic_var_type(input)?;
    Ok((input, BasicPredParType::BasicVarType(bvt)))
}
fn bppt_domain<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicPredParType, E> {
    let (input, domain) = domain(input)?;
    Ok((input, BasicPredParType::Domain(domain)))
}
#[derive(PartialEq, Clone, Debug)]
pub enum PredParType {
    Basic(BasicPredParType),
    Array {
        ix: PredIndexSet,
        par_type: BasicPredParType,
    },
}
fn pred_par_type<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PredParType, E> {
    let (input, ppt) = alt((ppt_basic_pred_par_type, array_of_pred_index_set))(input)?;
    Ok((input, ppt))
}
fn ppt_basic_pred_par_type<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PredParType, E> {
    let (input, bppt) = basic_pred_par_type(input)?;
    Ok((input, PredParType::Basic(bppt)))
}
fn array_of_pred_index_set<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PredParType, E> {
    let (input, _) = space0(input)?;
    let (input, _tag) = tag("array")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = char('[')(input)?;
    let (input, _) = space0(input)?;
    let (input, ix) = pred_index_set(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(']')(input)?;
    let (input, _) = space1(input)?;
    let (input, _tag) = tag("of")(input)?;
    let (input, _) = space1(input)?;
    let (input, par_type) = basic_pred_par_type(input)?;
    Ok((input, PredParType::Array { ix, par_type }))
}
#[derive(PartialEq, Clone, Debug)]
pub enum PredIndexSet {
    IndexSet(i128),
    Int,
}
fn pred_index_set<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PredIndexSet, E> {
    let (input, index_set) = alt((pis_int, pis_index_set))(input)?;
    Ok((input, index_set))
}
fn pis_int<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PredIndexSet, E> {
    let (input, _tag) = tag("int")(input)?;
    Ok((input, PredIndexSet::Int))
}
fn pis_index_set<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PredIndexSet, E> {
    let (input, int) = index_set(input)?;
    Ok((input, PredIndexSet::IndexSet(int)))
}
#[derive(PartialEq, Clone, Debug)]
pub enum BasicLiteralExpr {
    Bool(bool),
    Int(i128),
    Float(f64),
    Set(SetLiteral),
}
fn basic_literal_expr<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicLiteralExpr, E> {
    let (input, ble) = alt((
        ble_bool_literal,
        ble_set_literal,
        ble_float_literal,
        ble_int_literal,
    ))(input)?;
    Ok((input, ble))
}
fn ble_bool_literal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicLiteralExpr, E> {
    let (input, bl) = bool_literal(input)?;
    Ok((input, BasicLiteralExpr::Bool(bl)))
}
fn ble_int_literal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicLiteralExpr, E> {
    let (input, int) = int_literal(input)?;
    Ok((input, BasicLiteralExpr::Int(int)))
}
fn ble_float_literal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicLiteralExpr, E> {
    let (input, float) = float_literal(input)?;
    Ok((input, BasicLiteralExpr::Float(float)))
}
fn ble_set_literal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicLiteralExpr, E> {
    let (input, sl) = set_literal(input)?;
    Ok((input, BasicLiteralExpr::Set(sl)))
}

#[derive(PartialEq, Clone, Debug)]
pub enum BasicExpr {
    BasicLiteralExpr(BasicLiteralExpr),
    VarParIdentifier(String),
}
fn basic_expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BasicExpr, E> {
    let (input, expr) = alt((be_basic_literal_expr, be_var_par_identifier))(input)?;
    Ok((input, expr))
}
fn be_basic_literal_expr<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicExpr, E> {
    let (input, expr) = basic_literal_expr(input)?;
    Ok((input, BasicExpr::BasicLiteralExpr(expr)))
}
fn be_var_par_identifier<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BasicExpr, E> {
    let (input, id) = var_par_identifier(input)?;
    Ok((input, BasicExpr::VarParIdentifier(id)))
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    BasicExpr(BasicExpr),
    ArrayLiteral(ArrayLiteral),
}
fn expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Expr, E> {
    let (input, expr) = alt((e_basic_expr, e_array_literal))(input)?;
    Ok((input, expr))
}
fn e_basic_expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Expr, E> {
    let (input, basic_expr) = basic_expr(input)?;
    Ok((input, Expr::BasicExpr(basic_expr)))
}
fn e_array_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Expr, E> {
    let (input, array_literal) = array_literal(input)?;
    Ok((input, Expr::ArrayLiteral(array_literal)))
}
#[derive(PartialEq, Clone, Debug)]
pub enum ParExpr {
    BasicLiteralExpr(BasicLiteralExpr),
    ParArrayLiteral(ParArrayLiteral),
}
fn par_expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ParExpr, E> {
    let (input, expr) = alt((pe_basic_literal_expr, pe_par_array_literal))(input)?;
    Ok((input, expr))
}
fn pe_basic_literal_expr<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParExpr, E> {
    let (input, expr) = basic_literal_expr(input)?;
    Ok((input, ParExpr::BasicLiteralExpr(expr)))
}
fn pe_par_array_literal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParExpr, E> {
    let (input, expr) = par_array_literal(input)?;
    Ok((input, ParExpr::ParArrayLiteral(expr)))
}

#[derive(PartialEq, Clone, Debug)]
pub enum ParDeclItem {
    Basic {
        par_type: BasicParType,
        id: String,
        expr: ParExpr,
    },
    Array {
        ix: IndexSet,
        par_type: BasicParType,
        id: String,
        expr: ParExpr,
    },
}
#[test]
fn test_par_decl_item() {
    use nom::error::VerboseError;
    assert_eq!(
        par_decl_item::<VerboseError<&str>>("array [1..3] of float: X_139 = [1.0,1.0,1.0];"),
        Ok((
            "",
            ParDeclItem::Array {
                ix: IndexSet(3),
                par_type: BasicParType::Float,
                id: "X_139".to_string(),
                expr: ParExpr::ParArrayLiteral(vec![
                    BasicLiteralExpr::Float(1.0),
                    BasicLiteralExpr::Float(1.0),
                    BasicLiteralExpr::Float(1.0)
                ])
            }
        ))
    );
}
fn par_decl_item_ln<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParDeclItem, E> {
    let (input, item) = par_decl_item(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('\n')(input)?;
    Ok((input, item))
}
pub fn par_decl_item<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParDeclItem, E> {
    let (input, ptype) = par_type(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space0(input)?;
    let (input, id) = var_par_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = space0(input)?;
    let (input, expr) = par_expr(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(';')(input)?;
    match ptype {
        ParType::Array { ix, par_type } => Ok((
            input,
            ParDeclItem::Array {
                ix,
                par_type,
                id,
                expr,
            },
        )),
        ParType::Basic(par_type) => Ok((input, ParDeclItem::Basic { par_type, id, expr })),
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum VarDeclItem {
    Basic {
        var_type: BasicVarType,
        id: String,
        annos: Annotations,
        expr: Option<BasicExpr>,
    },
    Array {
        ix: IndexSet,
        var_type: BasicVarType,
        id: String,
        annos: Annotations,
        array_literal: ArrayLiteral,
    },
}
#[test]
fn test_var_decl_item() {
    use nom::error::VerboseError;
    assert_eq!(
        var_decl_item::<VerboseError<&str>>(
            "array [1..1] of var set of int: sets:: output_array([1..1]) = [X_0];"
        ),
        Ok((
            "",
            VarDeclItem::Array {
                ix: IndexSet(1),
                var_type: BasicVarType::SetOfInt,
                id: "sets".to_string(),
                annos: vec![Annotation::Id {
                    id: "output_array".to_string(),
                    expressions: vec![AnnExpr::Expr(Expr::ArrayLiteral(vec![
                        BasicExpr::BasicLiteralExpr(BasicLiteralExpr::Set(SetLiteral::IntRange(
                            1, 1
                        )))
                    ]))]
                }],
                array_literal: vec![BasicExpr::VarParIdentifier("X_0".to_owned())]
            }
        ))
    );
}
fn var_decl_item_ln<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, VarDeclItem, E> {
    let (input, item) = var_decl_item(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('\n')(input)?;
    Ok((input, item))
}
pub fn var_decl_item<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, VarDeclItem, E> {
    let (input, vdi) = alt((vdi_basic_var, vdi_array))(input)?;
    let (input, _tag) = space0(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, vdi))
}
fn vdi_basic_var<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, VarDeclItem, E> {
    let (input, var_type) = basic_var_type(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space0(input)?;
    let (input, id) = var_par_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, annos) = annotations(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = opt(char('='))(input)?;
    let (input, _) = space0(input)?;
    let (input, expr) = opt(basic_expr)(input)?;
    Ok((
        input,
        VarDeclItem::Basic {
            var_type,
            id,
            annos,
            expr,
        },
    ))
}
fn vdi_array<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, VarDeclItem, E> {
    // let (input, avt) = array_var_type(input)?;

    let (input, _tag) = tag("array")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = char('[')(input)?;
    let (input, _) = space0(input)?;
    let (input, int) = index_set(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(']')(input)?;
    let (input, _) = space1(input)?;
    let (input, _tag) = tag("of")(input)?;
    let (input, _) = space1(input)?;
    let (input, var_type) = basic_var_type(input)?;

    let (input, _) = space0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space0(input)?;
    let (input, id) = var_par_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, annos) = annotations(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = space0(input)?;
    let (input, array_literal) = array_literal(input)?;
    Ok((
        input,
        VarDeclItem::Array {
            ix: IndexSet(int),
            var_type,
            id,
            annos,
            array_literal,
        },
    ))
}
#[derive(PartialEq, Clone, Debug)]
pub struct ConstraintItem {
    pub id: String,
    pub exprs: Vec<Expr>,
    pub annos: Vec<Annotation>,
}
fn constraint_item_ln<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ConstraintItem, E> {
    let (input, item) = constraint_item(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('\n')(input)?;
    Ok((input, item))
}
pub fn constraint_item<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ConstraintItem, E> {
    let (input, _tag) = tag("constraint")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = identifier(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = space0(input)?;
    let (input, exprs) = separated_list(char(','), expr)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = space0(input)?;
    let (input, annos) = annotations(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, ConstraintItem { id, exprs, annos }))
}
#[derive(PartialEq, Clone, Debug)]
pub struct SolveItem {
    pub annotations: Annotations,
    pub goal: Goal,
}
fn solve_item_ln<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, SolveItem, E> {
    let (input, item) = solve_item(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('\n')(input)?;
    Ok((input, item))
}
pub fn solve_item<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, SolveItem, E> {
    let (input, _) = tag("solve")(input)?;
    let (input, _) = space1(input)?;
    let (input, annotations) = annotations(input)?;
    let (input, _) = space0(input)?;
    let (input, goal) = alt((satisfy, maximize, minimize))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, SolveItem { annotations, goal }))
}
#[derive(PartialEq, Clone, Debug)]
pub enum Goal {
    Satisfy,
    Minimize(BasicExpr),
    Maximize(BasicExpr),
}
fn satisfy<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Goal, E> {
    let (input, _) = tag("satisfy")(input)?;
    Ok((input, Goal::Satisfy))
}
fn maximize<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Goal, E> {
    let (input, _) = tag("maximize")(input)?;
    let (input, _) = space1(input)?;
    let (input, be) = basic_expr(input)?;
    Ok((input, Goal::Maximize(be)))
}
fn minimize<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Goal, E> {
    let (input, _) = tag("minimize")(input)?;
    let (input, _) = space1(input)?;
    let (input, be) = basic_expr(input)?;
    Ok((input, Goal::Minimize(be)))
}
type Annotations = Vec<Annotation>;
fn annotations<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Annotations, E> {
    let (input, annos) = many0(annotation1)(input)?;
    Ok((input, annos))
}
fn annotation1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Annotation, E> {
    let (input, _) = tag("::")(input)?;
    let (input, _) = space0(input)?;
    annotation(input)
}
#[derive(PartialEq, Clone, Debug)]
pub enum Annotation {
    Id {
        id: String,
        expressions: Vec<AnnExpr>,
    },
}
// <annotation> ::= <identifier>
//                | <identifier> "(" <ann-expr> "," ... ")"
fn annotation<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Annotation, E> {
    let (input, id) = identifier(input)?;
    let (input, we) = opt(char('('))(input)?;
    if we.is_some() {
        let (input, expressions_what) = separated_list(char(','), ann_expr)(input)?;
        let (input, _) = char(')')(input)?;
        Ok((
            input,
            Annotation::Id {
                id,
                expressions: expressions_what,
            },
        ))
    } else {
        let (input, _) = space0(input)?;
        Ok((
            input,
            Annotation::Id {
                id,
                expressions: vec![],
            },
        ))
    }
}
// <ann_expr>  := <expr> | <annotation>
// better
// <ann_expr> ::= <expr>
//              | <string_literal>
//              | "[" <annotation> "," ... "]"
#[derive(PartialEq, Clone, Debug)]
pub enum AnnExpr {
    Annotations(Annotations),
    String(String),
    Expr(Expr),
}
fn ann_expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, AnnExpr, E> {
    let (input, expr) = alt((ann_non_array_expr, ae_annotations))(input)?;
    Ok((input, expr))
}
fn ae_annotations<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, AnnExpr, E> {
    let (input, _) = char('[')(input)?;
    let (input, _) = space0(input)?;
    let (input, res) = separated_list(char(','), annotation)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, AnnExpr::Annotations(res)))
}
// ann_non_array_expr ::=
//       FZ_BOOL_LIT
//     | FZ_INT_LIT
//     | FZ_FLOAT_LIT
//     | set_literal
//     | var_par_id /* variable, possibly array */
//     | var_par_id '[' ann_non_array_expr ']' /* array access */
//     | FZ_STRING_LIT
fn ann_non_array_expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, AnnExpr, E> {
    let (input, expr) = alt((ae_expr, string_lit))(input)?;
    Ok((input, expr))
}
fn ae_expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, AnnExpr, E> {
    let (input, expr) = expr(input)?;
    Ok((input, AnnExpr::Expr(expr)))
}
#[test]
fn test_string_lit() {
    use nom::error::VerboseError;
    assert_eq!(
        string_lit::<VerboseError<&str>>("\"bla\""),
        Ok(("", AnnExpr::String("bla".to_string())))
    );
}
// TODO: implement support for escaped characters in string literals
fn string_lit<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, AnnExpr, E> {
    let (input, _) = char('"')(input)?;
    let (input, string) = take_while(is_valid)(input)?;
    let (input, _) = char('"')(input)?;
    Ok((input, AnnExpr::String(string.to_string())))
}
fn is_valid(c: char) -> bool {
    match c {
        '"' => false,
        _ => true,
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum SetLiteral {
    IntRange(i128, i128),
    FloatRange(f64, f64),
    SetFloats(Vec<f64>),
    SetInts(Vec<i128>), // possibly empty
}
fn set_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, SetLiteral, E> {
    let (input, sl) = alt((
        sl_int_range,
        sl_float_range,
        sl_set_of_floats,
        sl_set_of_ints,
    ))(input)?;
    Ok((input, sl))
}
fn sl_int_range<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, SetLiteral, E> {
    let (input, lb) = int_literal(input)?;
    let (input, _) = space0(input)?;
    let (input, _tag) = tag("..")(input)?;
    let (input, _) = space0(input)?;
    let (input, ub) = int_literal(input)?;
    Ok((input, SetLiteral::IntRange(lb, ub)))
}
fn sl_float_range<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, SetLiteral, E> {
    let (input, lb) = float_literal(input)?;
    let (input, _) = space0(input)?;
    let (input, _tag) = tag("..")(input)?;
    let (input, _) = space0(input)?;
    let (input, ub) = float_literal(input)?;
    Ok((input, SetLiteral::FloatRange(lb, ub)))
}
// "{" <int-literal> "," ... "}"
fn sl_set_of_ints<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, SetLiteral, E> {
    let (input, _) = char('{')(input)?;
    let (input, _) = space0(input)?;
    let (input, v) = separated_list(char(','), int_literal)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('}')(input)?;
    Ok((input, SetLiteral::SetInts(v)))
}
// "{" <float-literal> "," ... "}"
fn sl_set_of_floats<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, SetLiteral, E> {
    let (input, _) = char('{')(input)?;
    let (input, _) = space0(input)?;
    let (input, v) = separated_list(char(','), float_literal)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('}')(input)?;
    Ok((input, SetLiteral::SetFloats(v)))
}
type ArrayLiteral = Vec<BasicExpr>;
fn array_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ArrayLiteral, E> {
    let (input, _) = char('[')(input)?;
    let (input, _) = space0(input)?;
    let (input, al) = separated_list(char(','), basic_expr)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, al))
}
type ParArrayLiteral = Vec<BasicLiteralExpr>;
fn par_array_literal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParArrayLiteral, E> {
    let (input, _) = char('[')(input)?;
    let (input, _) = space0(input)?;
    let (input, v) = separated_list(char(','), basic_literal_expr)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, v))
}
fn identifier<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, String, E> {
    let (input, first) = one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)?;
    let (input, rest) = take_while(is_identifier_rest)(input)?;
    Ok((input, format!("{}{}", first, rest)))
}
fn var_par_identifier<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, String, E> {
    let (input, first) = one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")(input)?;
    let (input, rest) = take_while(is_identifier_rest)(input)?;
    Ok((input, format!("{}{}", first, rest)))
}
fn is_identifier_rest(c: char) -> bool {
    match c {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o'
        | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | 'A' | 'B' | 'C'
        | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q'
        | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' | '_' | '0' | '1' | '2' | '3'
        | '4' | '5' | '6' | '7' | '8' | '9' => true, //one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789")(input.into()) {
        _ => false,
    }
}
fn bool_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    let (input, string) = alt((tag("true"), tag("false")))(input)?;
    match string {
        "true" => Ok((input, true)),
        "false" => Ok((input, false)),
        x => panic!("unmatched bool literal {}", x),
    }
}
fn int_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i128, E> {
    let (input, int) = alt((decimal, hexadecimal, octal))(input)?;
    Ok((input, int as i128))
}
fn decimal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i128, E> {
    let (input, negation) = opt(char('-'))(input)?;
    let (input, int) = map_res(take_while1(is_dec_digit), from_dec)(input)?;
    if negation.is_some() {
        Ok((input, -(int as i128)))
    } else {
        Ok((input, int as i128))
    }
}
#[test]
fn test_hex() {
    use nom::error::VerboseError;
    assert_eq!(hexadecimal::<VerboseError<&str>>("-0x2f"), Ok(("", -47)));
}
fn hexadecimal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i128, E> {
    let (input, negation) = opt(char('-'))(input)?;
    let (input, _tag) = tag("0x")(input)?;
    let (input, int) = map_res(take_while1(is_hex_digit), from_hex)(input)?;
    if negation.is_some() {
        Ok((input, -(int as i128)))
    } else {
        Ok((input, int as i128))
    }
}
#[test]
fn test_oct() {
    use nom::error::VerboseError;
    assert_eq!(octal::<VerboseError<&str>>("0o21"), Ok(("", 17)));
}
fn octal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i128, E> {
    let (input, negation) = opt(char('-'))(input)?;
    let (input, _tag) = tag("0o")(input)?;
    let (input, int) = map_res(take_while1(is_oct_digit), from_oct)(input)?;
    if negation.is_some() {
        Ok((input, -(int as i128)))
    } else {
        Ok((input, int as i128))
    }
}
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}
fn from_oct(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 8)
}
fn is_oct_digit(c: char) -> bool {
    c.is_digit(8)
}
fn from_dec(input: &str) -> Result<u128, std::num::ParseIntError> {
    u128::from_str_radix(input, 10)
}
fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}
#[test]
fn test_float() {
    use nom::error::VerboseError;
    assert_eq!(
        float_literal::<VerboseError<&str>>("023.21"),
        Ok(("", 023.21))
    );
    assert_eq!(
        float_literal::<VerboseError<&str>>("0023.21E-098"),
        Ok(("", 0023.21E-098))
    );
    assert_eq!(
        float_literal::<VerboseError<&str>>("0023.21e+098"),
        Ok(("", 0023.21e+098))
    );
    assert_eq!(
        float_literal::<VerboseError<&str>>("002e+098"),
        Ok(("", 002e+098))
    );
    assert_eq!(float_literal::<VerboseError<&str>>("0.21"), Ok(("", 0.21)));
    assert_eq!(float_literal::<VerboseError<&str>>("1.0,"), Ok((",", 1.0)));
}
fn float_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, f64, E> {
    let (input, f) = double(input)?;
    Ok((input, f))
}
