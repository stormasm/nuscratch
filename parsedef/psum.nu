#### broken ** first parenthesis of def on 2nd line

def p1 [x]
{ echo $x }

def p2 [x]
{
echo $x }

def p3 [x]
{
echo $x
}

#### working

def p4 [x] { echo $x }

def p5 [x] {
echo $x }

def p6 [x] {
echo $x
}
