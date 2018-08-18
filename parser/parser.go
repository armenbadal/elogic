
//go:generate goyacc -p "ein" -o grammar.go grammar.y

package parser

import (
	"bufio"
	"github.com/armenbadal/elogic/ast"
	"os"
)

// կառուցվող միակ մոդուլը
var module *ast.Module
// կառուցվող ընթացիկ սխեման
var current *ast.Scheme

//
func ParseFile(fname string) *ast.Module {
	file, err := os.Open(fname)
	if err != nil {
		println("Cannot open file.")
		return nil
	}

	defer file.Close()
	
	module = ast.NewModule()

	lexer := &einLex{source: bufio.NewReader(file)}
	einParse(lexer)

	return module
}


