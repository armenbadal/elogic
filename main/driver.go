package main

import (
	"elogic/ast"
	"elogic/parser"
)

//
func main() {
	var m0 *ast.Module = parser.ParseFile("cases/case00.logic")
	println(m0.String())
}
