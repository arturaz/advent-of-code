package aoc

/** All bounds are inclusive. */
case class Bounds(minX: Int, maxX: Int, minY: Int, maxY: Int) {
  def min: Coords = Coords(minX, minY)
  def max: Coords = Coords(maxX, maxY)

  def expand(coords: Coords): Bounds = {
    Bounds(
      math.min(minX, coords.x),
      math.max(maxX, coords.x),
      math.min(minY, coords.y),
      math.max(maxY, coords.y)
    )
  }

  def coords: Iterator[Coords] = {
    for {
      x <- (minX to maxX).iterator
      y <- (minY to maxY).iterator
    } yield Coords(x, y)
  }
  
  /** Returns an iterator of iterators, where each inner iterator represents a line of coordinates. */
  def coordsByLine: Iterator[Iterator[Coords]] = {
    for {
      y <- (minY to maxY).iterator
    } yield (minX to maxX).iterator.map(x => Coords(x, y))
  }
}
object Bounds {
  def zero: Bounds = Bounds(0, 0, 0, 0)

  def from(coords: Iterator[Coords]): Bounds = {
    coords.foldLeft(Bounds.zero)((bounds, coord) => bounds.expand(coord))
  }
}