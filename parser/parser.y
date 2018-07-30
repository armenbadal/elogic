
%{
%}

%token xScheme
%token xEnd
%token xFalse
%token xTrue
%token xArrow
%token xNewLine
%token xIdent

%%
Module
    : SchemeList
    ;

SchemeList
    : SchemeList Scheme NewLines
    | /* empty */
    ;

Scheme
    : xScheme xIdent IdentList xArrow IdentList NewLines OperationList xEnd
    ;

IdentList
    : IdentList xIdent
    | xIdent
    ;

OperationList
    : OperationList Operation
    | Operation
    ;

Operation
    : xIdent IdentList xArrow IdentList NewLines
    ;

NewLines
    : NewLines xNewLine
    | xNewLines
    ;

