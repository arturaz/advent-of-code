package aoc

object Solution8 {
  enum Direction {
    case Left, Right

    override def toString: String = this match {
      case Left => "L"
      case Right => "R"
    }
  }
  object Direction {
    def parse(c: Char): Direction = c match {
      case 'L' => Left
      case 'R' => Right
    }
  }

  case class Node(id: String) extends AnyVal {
    override def toString: String = id
  }
  case class Entry(from: Node, left: Node, right: Node) {
    override def toString: String = s"$from = ($left, $right)"

    def apply(direction: Direction): Node = direction match {
      case Direction.Left => left
      case Direction.Right => right
    }
  }
  object Entry {
    // AAA = (BBB, BBB)
    def parse(line: String): Entry = {
      val from = Node(line.take(3))
      val left = Node(line.slice(7, 10))
      val right = Node(line.slice(12, 15))
      Entry(from, left, right)
    }
  }

  case class Data(entries: Vector[Entry], directions: Vector[Direction]) {
    lazy val directionsStream = LazyList.continually(directions).flatten

    lazy val asMap: Map[Node, Entry] = entries.map(e => e.from -> e).toMap

    def solve: Int = {
      var current = Node("AAA")
      var steps = 0
      val iter = directionsStream.iterator
      while (current != Node("ZZZ")) {
        val direction = iter.next()
        current = asMap(current)(direction)
        steps += 1
      }

      steps
    }
  }

  def parse(data: Vector[String]): Data = {
    val directions = data(0).iterator.map(Direction.parse).toVector
    val entries = data.iterator.drop(2).map(Entry.parse).toVector
    Data(entries, directions)
  }

  def run1(data: Vector[String]): String = {
    val parsed = parse(data)
    parsed.solve.toString
  }
}

object _8_1_Test1 extends Problem(8, InputMode.Test(1), Solution8.run1)
object _8_1_Test2 extends Problem(8, InputMode.Test(2), Solution8.run1)
object _8_1_Normal extends Problem(8, InputMode.Normal, Solution8.run1)