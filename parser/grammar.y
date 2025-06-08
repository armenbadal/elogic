
%{
package parser

import (
	"elogic/ast"
)

%}

%union {
  name string
  list []string
}


%token xScheme
%token xEnd
%token xArrow
%token xNewLine
%token xUnknown

%token <name> xIdent

%type <list> IdentList

%%
Module
  : OptNewLines SchemeList
  {
    return 0
  }
  ;

SchemeList
  : SchemeList Scheme
  | Scheme
  ;

Scheme
  : Header NewLines InstrList xEnd NewLines
  {
    module.Items = append(module.Items, current)
    current = nil
  }
  ;

Header
  : xScheme xIdent IdentList xArrow IdentList
  {
    current = ast.NewScheme($2, $3, $5)
  }
  ;

InstrList
  : InstrList Instruction
  | Instruction
  ;

Instruction
  : xIdent IdentList xArrow IdentList NewLines
  {
    ins := ast.NewInstruction($1, $2, $4)
    current.Body = append(current.Body, ins)
  }
  ;

IdentList
  : IdentList xIdent
  {
    $$ = append($1, $2)
  }
  | xIdent
  {
    $$ = make([]string, 0, 1)
    $$ = append($$, $1)
  }
  ;

NewLines
  : NewLines xNewLine
  | xNewLine
  ;

OptNewLines
  : NewLines
  | /* empty */
  ;
