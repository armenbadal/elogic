
package parser

import (
	"bufio"
	"unicode"
)


//
type einLex struct {
	source *bufio.Reader
}

//
const xEof = 0

//
func (l *einLex) next() rune {
	r, _, e := l.source.ReadRune()
	if e != nil {
		return xEof
	}
	
	return r
}

//
func (l *einLex) unread() {
	l.source.UnreadRune()
}

//
func (l *einLex) Lex(lval *einSymType) int {
	ch := l.next()

	for ch == ' ' || ch == '\t' {
		ch = l.next()
	}

	// մեկնաբանությունների տող
	if ch == ';' {
		for ch != '\n' {
			ch = l.next()
		}
	}

	if ch == xEof {
		return xEof
	}
	
	if ch == '\n' {
		return xNewLine
	}
	
	if unicode.IsLetter(ch) {
		s := ""
		for unicode.IsLetter(ch) || unicode.IsDigit(ch) {
			s = s + string(ch)
			ch = l.next()			
		}
		l.unread()

		if s == "SCHEME" {
			return xScheme
		} else if s == "END" {
			return xEnd
		}

		lval.name = s
		return xIdent
	}

	if ch == '-' {
		ch = l.next()
		if ch == '>' {
			return xArrow
		}
		l.unread()
		return xUnknown
	}
	
	return xUnknown
}

//
func (l *einLex) Error(s string) {
	println(s)
}


