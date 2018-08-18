
package main

import (
	"github.com/armenbadal/elogic/ast"
	"github.com/armenbadal/elogic/parser"
)


//
func main() {
	var m0 *ast.Module = parser.ParseFile("cases/case00.logic")
	println(m0.String())
}


