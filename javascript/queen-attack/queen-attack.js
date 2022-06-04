//
// This is only a SKELETON file for the 'Queen Attack' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export class QueenAttack {
  constructor({ black = [0, 3], white = [7, 3] } = {}) {
    this.whiteQueen = new Queen(white);
    this.blackQueen = new Queen(black);

    if (this.whiteQueen.isPosition(this.blackQueen.position))
      throw new Error("Queens cannot share the same space");
  }

  get white() {
    return this.whiteQueen.position;
  }

  get black() {
    return this.blackQueen.position;
  }

  toString() {
    const emptyRow = Array(8).fill("_");
    const board = Array(8);
    for (let i = 0; i < 8; i++) {
      board[i] = [...emptyRow];
    }

    board[this.whiteQueen.row][this.whiteQueen.column] = "W";
    board[this.blackQueen.row][this.blackQueen.column] = "B";

    return board
      .reverse()
      .map((row) => row.join(" "))
      .reverse()
      .join("\n");
  }

  get canAttack() {
    return (
      this.areOnSameRow() || this.areOnSameColumn() || this.areOnSameDiagonal()
    );
  }

  areOnSameDiagonal() {
    return (
      this.whiteQueen.verticalDistanceFrom(this.blackQueen.position) ===
      this.whiteQueen.horizontalDistanceFrom(this.blackQueen.position)
    );
  }

  areOnSameColumn() {
    return this.whiteQueen.isOnColumn(this.blackQueen.column);
  }

  areOnSameRow() {
    return this.whiteQueen.isOnRow(this.blackQueen.row);
  }
}

class Queen {
  constructor([row, column]) {
    if (row < 0 || row > 7 || column < 0 || column > 7)
      throw new Error("Queen must be placed on the board");

    this.row = row;
    this.column = column;
  }

  get position() {
    return [this.row, this.column];
  }

  isPosition(positionToCheck) {
    return (
      this.row === positionToCheck[0] && this.column === positionToCheck[1]
    );
  }

  isOnRow(rowToCheck) {
    return this.row === rowToCheck;
  }

  isOnColumn(columnToCheck) {
    return this.column === columnToCheck;
  }

  verticalDistanceFrom(position) {
    const row = position[0];
    return Math.abs(this.row - row);
  }

  horizontalDistanceFrom(position) {
    const column = position[1];
    return Math.abs(this.column - column);
  }
}
