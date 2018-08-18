
%{
package parser

import "container/list"
%}

%union {
    name string
    list *list.List
}

%token xScheme
%token xEnd
%token xFalse
%token xTrue
%token xArrow
%token xNewLine
%token <name> xIdent

%type <list> IdentList

%%
Module
    : SchemeList
    ;

SchemeList
    : SchemeList Scheme NewLines
    | /* empty */
    ;

Scheme
    : Header NewLines OperationList xEnd
    ;

Header
    : xScheme xIdent IdentList xArrow IdentList
    ;

IdentList
    : IdentList xIdent
    {
        $$ = nil
    }
    | xIdent
    {
        $$ = nil
    }
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
    | xNewLine
    ;

