package aoc

case class Coords(x: Int, y: Int) {
  def up: Coords = up(1)
  def up(n: Int): Coords = Coords(x, y - n)

  def down: Coords = down(1)
  def down(n: Int): Coords = Coords(x, y + n)

  def left: Coords = left(1)
  def left(n: Int): Coords = Coords(x - n, y)

  def right: Coords = right(1)
  def right(n: Int): Coords = Coords(x + n, y)

  def around: Iterator[Coords] = Iterator(up, down, left, right)
  def diagonals: Iterator[Coords] = Iterator(up.left, up.right, down.left, down.right)
  def aroundWithDiagonals: Iterator[Coords] = around ++ diagonals
}