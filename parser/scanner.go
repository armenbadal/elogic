
package parser

import (
	"bufio"
	"unicode"
)

type eloLex struct {
	source *bufio.Reader
	ch rune
}

//
func (ex *eloLex) next() {
	r, _, e := ex.source.ReadRune()
	if e != nil {
		ex.ch = 0
	} else {
		ex.ch = r
	}
}

//
func (ex *eloLex) Lex(lval *eloSymType) int {
	for ex.ch == ' ' || ex.ch == '\t' {
		ex.next()
	}

	if ex.ch == 0 {
		return 0
	}

	if unicode.IsLetter(ex.ch) {
		lex := ""
		for unicode.IsLetter(ex.ch) || unicode.IsDigit(ex.ch) {
			lex = lex + string(ex.ch)
			ex.next()
		}
		if lex == "SCHEME" {
			return xScheme
		}
		if lex == "END" {
			return xEnd
		}
		lval.name = lex
		return xIdent
	}

	return 0
}

