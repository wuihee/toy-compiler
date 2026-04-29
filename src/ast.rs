pub struct Program {
    pub main: MainClass,
    pub classes: Vec<Class>,
}

pub struct MainClass {
    pub name: Identifier,
    pub parameter: Identifier,
    pub statement: Statement,
}

pub struct Class {
    pub name: Identifier,
    pub super_class: Option<Identifier>,
    pub fields: Vec<Variable>,
    pub methods: Vec<Method>,
}

pub struct Method {
    pub return_type: Type,
    pub name: Identifier,
    pub parameters: Vec<Variable>,
    pub variables: Vec<Variable>,
    pub body: Vec<Statement>,
    pub return_expression: Expression,
}

pub struct Variable {
    pub ty: Type,
    pub name: Identifier,
}

pub struct Identifier(String);

pub enum Expression {
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    ArrayLength {
        array: Box<Expression>,
    },
    ArrayLookup {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    BooleanLiteral(bool),
    Call {
        receiver: Box<Expression>,
        method: Identifier,
        args: Vec<Expression>,
    },
    StringLiteral(String),
    IntegerLiteral(i32),
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Minus {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    NewArray {
        length: Box<Expression>,
    },
    NewObject {
        name: Identifier,
    },
    Not {
        operand: Box<Expression>,
    },
    Plus {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    This,
    Times {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

pub enum Statement {
    ArrayAssign {
        array: Identifier,
        index: Expression,
        value: Expression,
    },
    Assign {
        target: Identifier,
        value: Expression,
    },
    Block {
        statements: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Box<Statement>,
    },
    Print {
        expression: Expression,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
}

pub enum Type {
    Boolean,
    String,
    IntegerArray,
    Integer,
}
