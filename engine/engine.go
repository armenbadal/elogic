
package engine

/**/
type Pin struct {
	name string
}

/**/
type Operation struct {
	scheme *Scheme
	inputs, outputs []*Pin
}

/**/
type Scheme struct {
	name string
	inputs, outputs []string
	body []*Operation
}

/**/
type Module struct {
	schemes []*Scheme
}

