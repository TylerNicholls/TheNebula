Tim and Tyler and Michael's game, the nebula.
Written in rust.

## things of import:
it has some circles.
they move.

# this is a heading?
more words

### Smaller?
* is this smaller?
* another asterisk


### Tyler's Notes:
d=√​(x​2​​−x​1​​)​2​​+(y​2​​−y​1​​)​2​​​​​

if d < (Pradius + Eradius){

    collision(P, E);
}

collision(player P, enemy E){

int collisionX = (P.xCoord + E.xCoord)/2 -(Pradius-Eradius);
int collisionY =(P.yCoord + E.yCoord)/2; - (Pradius - Eradius);

double Pangle = tan-1(collisionY / collisionX);
double Eangle = 90 - Pangle;
