//! Abstract Syntax Tree
//!
//! The types in this module mirror the MiniJava grammar: a [`Program`] is a
//! [`MainClass`] plus zero or more regular [`Class`]es, each class has fields
//! and methods, and method bodies are built from [`Statement`]s and
//! [`Expression`]s.

/// A complete MiniJava program: one main class followed by zero or more
/// regular classes.
#[derive(Clone, Debug)]
pub struct Program {
    pub main: MainClass,
    pub classes: Vec<Class>,
}

/// MiniJava's main class.
///
/// Every program has exactly one, containing a single `public static void
/// main(String[] args)` method whose body is a single statement.
#[derive(Clone, Debug)]
pub struct MainClass {
    pub name: Identifier,

    /// The single statement that forms the body of `main`.
    pub body: Statement,
}

/// A class declaration.
#[derive(Clone, Debug)]
pub struct Class {
    pub name: Identifier,

    /// The explicit superclass, or `None` if the class has no `extends`.
    pub super_class: Option<Identifier>,

    pub fields: Vec<Variable>,
    pub methods: Vec<Method>,
}

/// A method declaration.
#[derive(Clone, Debug)]
pub struct Method {
    pub return_type: Type,
    pub name: Identifier,
    pub parameters: Vec<Variable>,

    /// Local variables declared at the top of the method body, distinct from
    /// [`parameters`](Self::parameters) and from the enclosing class's fields.
    pub variables: Vec<Variable>,

    pub body: Vec<Statement>,

    /// The expression returned by the method. MiniJava methods always end in
    /// a single `return` expression, so this is a required field.
    pub return_expression: Expression,
}

/// A typed name binding used for fields, method parameters, and locals.
#[derive(Clone, Debug)]
pub struct Variable {
    pub ty: Type,
    pub name: Identifier,
}

/// A MiniJava identifier.
///
/// Wrapping `String` in a newtype keeps identifiers from being mixed up with
/// arbitrary strings.
#[derive(Clone, Debug)]
pub struct Identifier(pub String);

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier {
    /// Construct an `Identifier` from anything convertible into a `String`.
    pub fn new(identifier: impl Into<String>) -> Self {
        Identifier(identifier.into())
    }

    /// Borrow the identifier as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An expression - anything that evaluates to a value.
#[derive(Clone, Debug)]
pub enum Expression {
    /// `left && right`
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// `array.length`
    ArrayLength { array: Box<Expression> },

    /// `array[index]`
    ArrayLookup {
        array: Box<Expression>,
        index: Box<Expression>,
    },

    /// A boolean literal: `true` or `false`.
    BooleanLiteral(bool),

    /// `receiver.method(args)`
    Call {
        receiver: Box<Expression>,
        method: Identifier,
        args: Vec<Expression>,
    },

    /// A string literal like `"hello"`.
    StringLiteral(String),

    /// An integer literal like `42`.
    IntegerLiteral(i64),

    /// `left < right`
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// `left - right`
    Minus {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// `new int[length]`
    NewArray { length: Box<Expression> },

    /// `new name()`
    NewObject { name: Identifier },

    /// `!operand`
    Not { operand: Box<Expression> },

    /// `left + right`
    Plus {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// `this`
    This,

    /// `left * right`
    Times {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

/// A statement - anything that executes for its effect rather than producing
/// a value.
#[derive(Clone, Debug)]
pub enum Statement {
    /// `array[index] = value`
    ArrayAssign {
        array: Identifier,
        index: Expression,
        value: Expression,
    },

    /// `target = value`
    Assign {
        target: Identifier,
        value: Expression,
    },

    /// A braced block: `{ stmt; stmt; ... }`.
    Block { statements: Vec<Statement> },

    /// `if (condition) then_branch else else_branch`. MiniJava requires the
    /// `else` branch so it is not optional.
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Box<Statement>,
    },

    /// `System.out.println(expression)`
    Print { expression: Expression },

    /// `while (condition) body`
    While {
        condition: Expression,
        body: Box<Statement>,
    },
}

/// A MiniJava type.
#[derive(Clone, Debug)]
pub enum Type {
    /// `boolean`
    Boolean,

    /// `String`. Only used as the elemtn type of `String[]` in `main`'s
    /// signature; not usable type elsewhere in MiniJava.
    String,

    /// `int[]`
    IntegerArray,

    /// `int`
    Integer,
}
