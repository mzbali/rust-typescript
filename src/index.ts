enum Colour {
    Red,
    Green,
    Blue,
    Yellow
}

function PrintColour(colour) {
    switch (colour) {
        case Colour.Red:
            console.log("red");
            break;
        case Colour.Green:
            console.log("green");
            break;
        case Colour.Blue:
            console.log("blue");
            break;
    }
}
PrintColour(Colour.Yellow);
