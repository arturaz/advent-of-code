package aoc

object Solution4 {
  case class Card(id: Int, winning: Set[Int], numbers: Vector[Int]) {
    lazy val noOfWinning: Int = numbers.count(winning.contains)

    /** 1st winning number gives one point, then each one doubles that. */
    lazy val score: Int = {
      if (noOfWinning <= 0) 0
      else if (noOfWinning == 1) 1
      else math.pow(2, noOfWinning - 1).toInt
    }

    /**
     * Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if
     * card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.
     */
    def winningNumbers: Iterator[Int] = (1 to noOfWinning).iterator.map(id + _)
  }

  // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
  def parseLine(line: String): Card = {
    val Array(id, numbers) = line.split(": ")
    val Array(winning, numbers2) = numbers.split(" \\| ")
    val winningSet = winning.trim.split("\\s+").iterator.map(_.toInt).toSet
    val numbersVector = numbers2.trim.split("\\s+").iterator.map(_.toInt).toVector
    Card(id.split("\\s+")(1).toInt, winningSet, numbersVector)
  }

  def run1(data: Vector[String]): String = {
    val result = data.map(parseLine).map(_.score).sum
    result.toString
  }

  def run2Slow(data: Vector[String]): String = {
    val cards = data.iterator.map(parseLine).map(c => c.id -> c).toMap

    var totalCards = 0
    val unchecked = collection.mutable.Queue.from(cards.valuesIterator)
    while (unchecked.nonEmpty) {
      val card = unchecked.dequeue()
      totalCards += 1

      val won = card.winningNumbers.toVector
      if (won.nonEmpty) {
        val copied = won.map(cards)

        unchecked ++= copied
      }
    }

    totalCards.toString
  }

  def run2Fast(data: Vector[String]): String = {
    val cards = data.iterator.map(parseLine).map(c => c.id -> c).toMap

    val ids = cards.keysIterator.toVector.sorted
    case class WithCount[A](value: A, count: Int)

    var totalCards = 0
    var cardsWithCounts = cards.map { case (id, card) => id -> WithCount(card, 1) }
    ids.foreach { id =>
      val WithCount(card, count) = cardsWithCounts(id)
      totalCards += count

      val won = card.winningNumbers.toVector
      if (won.nonEmpty) {
        won.foreach { wonId =>
          cardsWithCounts = cardsWithCounts.updatedWith(wonId) {
            case None => ??? // should not happen
            case Some(WithCount(card, current)) => Some(WithCount(card, current + count))
          }
        }
      }
    }

    totalCards.toString
  }
}

object _4_1_Test extends Problem(4, InputMode.Test(1), Solution4.run1)
object _4_1_Normal extends Problem(4, InputMode.Normal, Solution4.run1)

val run2Fns = ToRun(
  RunFn("slow", Solution4.run2Slow),
  RunFn("fast", Solution4.run2Fast),
)
object _4_2_Test extends Problem(4, InputMode.Test(1), run2Fns)
object _4_2_Normal extends Problem(4, InputMode.Normal, run2Fns)
