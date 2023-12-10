package aoc

import org.locationtech.jts.geom.Coordinate

import scala.annotation.targetName

case class Coords(x: Int, y: Int) {
  def up: Coords = up(1)
  def up(n: Int): Coords = Coords(x, y - n)

  def down: Coords = down(1)
  def down(n: Int): Coords = Coords(x, y + n)

  def left: Coords = left(1)
  def left(n: Int): Coords = Coords(x - n, y)

  def right: Coords = right(1)
  def right(n: Int): Coords = Coords(x + n, y)

  def move(direction: Direction): Coords = direction match {
    case Direction.Up => up
    case Direction.Down => down
    case Direction.Left => left
    case Direction.Right => right
  }
  @targetName("plus") def +(direction: Direction): Coords = move(direction)

  def around: Iterator[Coords] = Iterator(up, down, left, right)
  def diagonals: Iterator[Coords] = Iterator(up.left, up.right, down.left, down.right)
  def aroundWithDiagonals: Iterator[Coords] = around ++ diagonals
  
  def asJTS: Coordinate = new Coordinate(x, y)
}

case class WithCoords[A](coords: Coords, value: A) {
  def asTuple: (Coords, A) = (coords, value)
}
object WithCoords {
  def fromTuple[A](tuple: (Coords, A)): WithCoords[A] = WithCoords(tuple._1, tuple._2)
}