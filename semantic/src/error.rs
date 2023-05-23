use frontend::ast::Pos;

pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

pub enum ErrorKind {
    NonExistentVariable(String),
    NonExistentFunction(String),

    VariableRedefinitionError {
        first_declaration: Pos,
        id: String,
    },

    FunctionRedefinitionError {
        first_declaration: Pos,
        id: String,
    },

    InvalidArgumentsCount {
        expected: usize,
        received: usize,
        function_id: String,
    },

    DuplicateFunctionArguments(String),

    ReturnOutsideFunction,
    ExpectedLValue,
}

impl Error {
    pub fn variable_redefinition(id: frontend::ast::Id, first_declaration: Pos) -> Self {
        let kind = ErrorKind::VariableRedefinitionError {
            first_declaration,
            id: id.value,
        };

        Error { kind, pos: id.pos }
    }

    pub fn function_redefinition(id: frontend::ast::Id, first_declaration: Pos) -> Self {
        let kind = ErrorKind::VariableRedefinitionError {
            first_declaration,
            id: id.value,
        };

        Error { kind, pos: id.pos }
    }

    pub fn non_existent_variable(id: frontend::ast::Id) -> Self {
        let kind = ErrorKind::NonExistentVariable(id.value);

        Error { kind, pos: id.pos }
    }

    pub fn non_existent_function(id: frontend::ast::Id) -> Self {
        let kind = ErrorKind::NonExistentFunction(id.value);

        Error { kind, pos: id.pos }
    }

    pub fn expected_lvalue() -> Self {
        let kind = ErrorKind::ExpectedLValue;

        Error {
            kind,
            pos: Pos::default(),
        }
    }

    pub fn duplicate_function_args(id: frontend::ast::Id) -> Self {
        let kind = ErrorKind::DuplicateFunctionArguments(id.value);

        Self { kind, pos: id.pos }
    }

    pub fn return_outside_function() -> Self {
        let kind = ErrorKind::ReturnOutsideFunction;

        Self {
            kind,
            pos: Pos::default(),
        }
    }
}
