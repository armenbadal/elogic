
package ast


import (
	"container/list"
	"fmt"
	"strings"
)


type Module struct {
	Items *list.List
}

func NewModule() *Module {
	return &Module{Items: list.New()}
}

type Scheme struct {
	Name string
	Inputs, Outputs []string
	Body *list.List
}

func NewScheme(nm string, ins, outs []string) *Scheme {
	return &Scheme{Name: nm, Inputs: ins, Outputs: outs, Body: list.New()}
}

type Instruction struct {
	Name string
	Inputs, Outputs []string
}

func NewInstruction(nm string, ins, outs []string) *Instruction {
	return &Instruction{Name: nm, Inputs: ins, Outputs: outs}
}

//
func (mp *Module) String() string {
	var br strings.Builder
	var ser = func( s interface{} ) { 
		br.WriteString(s.(*Scheme).String())
	}
	forEach(mp.Items, ser)
	return br.String()
}

//
func (sp *Scheme) String() string {
	var br strings.Builder

	ins := strings.Join(sp.Inputs, " ")
	outs := strings.Join(sp.Outputs, " ")
	br.WriteString(fmt.Sprintf("SCHEME %s %s -> %s\n", sp.Name, ins, outs))

	ser := func(s interface{}) { 
		br.WriteString(s.(*Instruction).String())
	}
	forEach(sp.Body, ser)

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
