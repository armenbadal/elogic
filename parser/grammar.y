
%{
package parser

import (
    "container/list"
	"github.com/armenbadal/elogic/ast"
)

%}

%union {
  name string
  list *list.List
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
		module.Items.PushBack(current)
		current = nil
    }
    ;

Header
    : xScheme xIdent IdentList xArrow IdentList
    {
		current = ast.NewScheme($2, toSlice($3), toSlice($5))
    }
    ;

InstrList
    : InstrList Instruction
    | Instruction
    ;

Instruction
    : xIdent IdentList xArrow IdentList NewLines
    {
		ins := ast.NewInstruction($1, toSlice($2), toSlice($4))
        current.Body.PushBack(ins)
    }
    ;

IdentList
    : IdentList xIdent
    {
		$$ = $1
		$$.PushBack($2)
    }
    | xIdent
    {
		$$ = list.New()
		$$.PushBack($1)
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

%%

func toSlice(sil *list.List) []string {
    res := make([]string, sil.Len())
	i := 0
	ei := sil.Front()
	for ei != nil {
		res[i] = ei.Value.(string)
		i++
		ei = ei.Next()
	}
	return res
}


