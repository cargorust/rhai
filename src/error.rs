//! Module containing error definitions for the parsing process.

use crate::parser::Position;

use crate::stdlib::{char, error::Error, fmt, string::String};

/// Error when tokenizing the script text.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum LexError {
    /// An unexpected character is encountered when tokenizing the script text.
    UnexpectedChar(char),
    /// A string literal is not terminated before a new-line or EOF.
    UnterminatedString,
    /// An string/character/numeric escape sequence is in an invalid format.
    MalformedEscapeSequence(String),
    /// An numeric literal is in an invalid format.
    MalformedNumber(String),
    /// An character literal is in an invalid format.
    MalformedChar(String),
    /// An identifier is in an invalid format.
    MalformedIdentifier(String),
}

impl Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(c) => write!(f, "Unexpected '{}'", c),
            Self::MalformedEscapeSequence(s) => write!(f, "Invalid escape sequence: '{}'", s),
            Self::MalformedNumber(s) => write!(f, "Invalid number: '{}'", s),
            Self::MalformedChar(s) => write!(f, "Invalid character: '{}'", s),
            Self::MalformedIdentifier(s) => write!(f, "Variable name is not proper: '{}'", s),
            Self::UnterminatedString => write!(f, "Open string is not terminated"),
        }
    }
}

/// Type of error encountered when parsing a script.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseErrorType {
    /// Error in the script text. Wrapped value is the error message.
    BadInput(String),
    /// The script ends prematurely.
    UnexpectedEOF,
    /// An unknown operator is encountered. Wrapped value is the operator.
    UnknownOperator(String),
    /// Expecting a particular token but not finding one. Wrapped values are the token and description.
    MissingToken(String, String),
    /// An expression in function call arguments `()` has syntax error. Wrapped value is the error description (if any).
    MalformedCallExpr(String),
    /// An expression in indexing brackets `[]` has syntax error. Wrapped value is the error description (if any).
    ///
    /// Not available under the `no_index` feature.
    #[cfg(not(feature = "no_index"))]
    MalformedIndexExpr(String),
    /// A map definition has duplicated property names. Wrapped value is the property name.
    ///
    /// Not available under the `no_object` feature.
    #[cfg(not(feature = "no_object"))]
    DuplicatedProperty(String),
    /// Invalid expression assigned to constant. Wrapped value is the name of the constant.
    ForbiddenConstantExpr(String),
    /// Missing a property name for custom types and maps.
    PropertyExpected,
    /// Missing a variable name after the `let`, `const` or `for` keywords.
    VariableExpected,
    /// Missing an expression. Wrapped value is the expression type.
    ExprExpected(String),
    /// Defining a function `fn` in an appropriate place (e.g. inside another function).
    ///
    /// Not available under the `no_function` feature.
    #[cfg(not(feature = "no_function"))]
    WrongFnDefinition,
    /// Missing a function name after the `fn` keyword.
    ///
    /// Not available under the `no_function` feature.
    #[cfg(not(feature = "no_function"))]
    FnMissingName,
    /// A function definition is missing the parameters list. Wrapped value is the function name.
    ///
    /// Not available under the `no_function` feature.
    #[cfg(not(feature = "no_function"))]
    FnMissingParams(String),
    /// A function definition has duplicated parameters. Wrapped values are the function name and parameter name.
    ///
    /// Not available under the `no_function` feature.
    #[cfg(not(feature = "no_function"))]
    FnDuplicatedParam(String, String),
    /// A function definition is missing the body. Wrapped value is the function name.
    ///
    /// Not available under the `no_function` feature.
    #[cfg(not(feature = "no_function"))]
    FnMissingBody(String),
    /// Assignment to an inappropriate LHS (left-hand-side) expression.
    AssignmentToInvalidLHS,
    /// Assignment to a copy of a value.
    AssignmentToCopy,
    /// Assignment to an a constant variable.
    AssignmentToConstant(String),
    /// Break statement not inside a loop.
    LoopBreak,
}

impl ParseErrorType {
    /// Make a `ParseError` using the current type and position.
    pub(crate) fn into_err(self, pos: Position) -> ParseError {
        ParseError(self, pos)
    }

    /// Make a `ParseError` using the current type and EOF position.
    pub(crate) fn into_err_eof(self) -> ParseError {
        ParseError(self, Position::eof())
    }
}

/// Error when parsing a script.
#[derive(Debug, PartialEq, Clone)]
pub struct ParseError(pub(crate) ParseErrorType, pub(crate) Position);

impl ParseError {
    /// Get the parse error.
    pub fn error_type(&self) -> &ParseErrorType {
        &self.0
    }

    /// Get the location in the script of the error.
    pub fn position(&self) -> Position {
        self.1
    }

    pub(crate) fn desc(&self) -> &str {
        match &self.0 {
            ParseErrorType::BadInput(p) => p,
            ParseErrorType::UnexpectedEOF => "Script is incomplete",
            ParseErrorType::UnknownOperator(_) => "Unknown operator",
            ParseErrorType::MissingToken(_, _) => "Expecting a certain token that is missing",
            ParseErrorType::MalformedCallExpr(_) => "Invalid expression in function call arguments",
            #[cfg(not(feature = "no_index"))]
            ParseErrorType::MalformedIndexExpr(_) => "Invalid index in indexing expression",
            #[cfg(not(feature = "no_object"))]
            ParseErrorType::DuplicatedProperty(_) => "Duplicated property in object map literal",
            ParseErrorType::ForbiddenConstantExpr(_) => "Expecting a constant",
            ParseErrorType::PropertyExpected => "Expecting name of a property",
            ParseErrorType::VariableExpected => "Expecting name of a variable",
            ParseErrorType::ExprExpected(_) => "Expecting an expression",
            #[cfg(not(feature = "no_function"))]
            ParseErrorType::FnMissingName => "Expecting name in function declaration",
            #[cfg(not(feature = "no_function"))]
            ParseErrorType::FnMissingParams(_) => "Expecting parameters in function declaration",
            #[cfg(not(feature = "no_function"))]
            ParseErrorType::FnDuplicatedParam(_,_) => "Duplicated parameters in function declaration",
            #[cfg(not(feature = "no_function"))]
            ParseErrorType::FnMissingBody(_) => "Expecting body statement block for function declaration",
            #[cfg(not(feature = "no_function"))]
            ParseErrorType::WrongFnDefinition => "Function definitions must be at global level and cannot be inside a block or another function",
            ParseErrorType::AssignmentToInvalidLHS => "Cannot assign to this expression",
            ParseErrorType::AssignmentToCopy => "Cannot assign to this expression because it will only be changing a copy of the value",
            ParseErrorType::AssignmentToConstant(_) => "Cannot assign to a constant variable.",
            ParseErrorType::LoopBreak => "Break statement should only be used inside a loop"
        }
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ParseErrorType::BadInput(s) | ParseErrorType::MalformedCallExpr(s) => {
                write!(f, "{}", if s.is_empty() { self.desc() } else { s })?
            }
            ParseErrorType::ForbiddenConstantExpr(s) => {
                write!(f, "Expecting a constant to assign to '{}'", s)?
            }
            ParseErrorType::UnknownOperator(s) => write!(f, "{}: '{}'", self.desc(), s)?,

            #[cfg(not(feature = "no_index"))]
            ParseErrorType::MalformedIndexExpr(s) => {
                write!(f, "{}", if s.is_empty() { self.desc() } else { s })?
            }

            #[cfg(not(feature = "no_object"))]
            ParseErrorType::DuplicatedProperty(s) => {
                write!(f, "Duplicated property '{}' for object map literal", s)?
            }

            ParseErrorType::ExprExpected(s) => write!(f, "Expecting {} expression", s)?,

            #[cfg(not(feature = "no_function"))]
            ParseErrorType::FnMissingParams(s) => {
                write!(f, "Expecting parameters for function '{}'", s)?
            }

            #[cfg(not(feature = "no_function"))]
            ParseErrorType::FnMissingBody(s) => {
                write!(f, "Expecting body statement block for function '{}'", s)?
            }

            #[cfg(not(feature = "no_function"))]
            ParseErrorType::FnDuplicatedParam(s, arg) => {
                write!(f, "Duplicated parameter '{}' for function '{}'", arg, s)?
            }

            ParseErrorType::MissingToken(token, s) => write!(f, "Expecting '{}' {}", token, s)?,

            ParseErrorType::AssignmentToConstant(s) if s.is_empty() => {
                write!(f, "{}", self.desc())?
            }
            ParseErrorType::AssignmentToConstant(s) => {
                write!(f, "Cannot assign to constant '{}'", s)?
            }
            _ => write!(f, "{}", self.desc())?,
        }

        if !self.1.is_eof() {
            write!(f, " ({})", self.1)
        } else if !self.1.is_none() {
            // Do not write any position if None
            Ok(())
        } else {
            write!(f, " at the end of the script but there is no more input")
        }
    }
}
