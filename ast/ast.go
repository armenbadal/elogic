package ast

import (
	"container/list"
	"fmt"
	"strings"
)

type Module struct {
	Items []*Scheme
}

func NewModule() *Module {
	return &Module{Items: make([]*Scheme, 0, 8)}
}

type Scheme struct {
	Name            string
	Inputs, Outputs []string
	Body            []*Instruction
}

func NewScheme(nm string, ins, outs []string) *Scheme {
	return &Scheme{
		Name:    nm,
		Inputs:  ins,
		Outputs: outs,
		Body:    make([]*Instruction, 0, 8)}
}

type Instruction struct {
	Name            string
	Inputs, Outputs []string
}

func NewInstruction(nm string, ins, outs []string) *Instruction {
	return &Instruction{Name: nm, Inputs: ins, Outputs: outs}
}

//
func (mp *Module) String() string {
	var br strings.Builder
	for _, elem := range mp.Items {
		br.WriteString(elem.String())
	}
	return br.String()
}

//
func (sp *Scheme) String() string {
	var br strings.Builder

	ins := strings.Join(sp.Inputs, " ")
	outs := strings.Join(sp.Outputs, " ")
	br.WriteString(fmt.Sprintf("SCHEME %s %s -> %s\n", sp.Name, ins, outs))

	for _, elem := range sp.Body {
		br.WriteString(elem.String())
	}

	br.WriteString("END\n\n")

	return br.String()
}

//
func (ip *Instruction) String() string {
	ins := strings.Join(ip.Inputs, " ")
	outs := strings.Join(ip.Outputs, " ")
	return fmt.Sprintf("  %s %s -> %s\n", ip.Name, ins, outs)
}

//
func forEach(l *list.List, f func(interface{})) {
	for ei := l.Front(); ei != nil; ei = ei.Next() {
		f(ei.Value)
	}
}

//
func mapCar(l *list.List, f func(interface{}) interface{}) *list.List {
	res := list.New()
	forEach(l, func(e interface{}) { res.PushBack(f(e)) })
	return res
}
