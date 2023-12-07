package aoc

import cats.{Comparison, Order}
import cats.syntax.all.*

object Solution7 {
  enum Card {
    case Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two

    override def toString = this match {
      case Ace => "A"
      case King => "K"
      case Queen => "Q"
      case Jack => "J"
      case Ten => "T"
      case Nine => "9"
      case Eight => "8"
      case Seven => "7"
      case Six => "6"
      case Five => "5"
      case Four => "4"
      case Three => "3"
      case Two => "2"
    }
  }
  object Card {
    given Order[Card] = Order.by(-_.ordinal)

    val part2IndividualOrder: Order[Card] = Order.by {
      case Jack => Int.MinValue // Jack is treated as the lowest card
      case card => -card.ordinal
    }

    def parse(c: Char): Card = c match {
      case 'A' => Ace
      case 'K' => King
      case 'Q' => Queen
      case 'J' => Jack
      case 'T' => Ten
      case '9' => Nine
      case '8' => Eight
      case '7' => Seven
      case '6' => Six
      case '5' => Five
      case '4' => Four
      case '3' => Three
      case '2' => Two
    }
  }

  enum HandType {
    case FiveOfAKind, FourOfAKind, FullHouse, ThreeOfAKind, TwoPair, OnePair, HighCard
  }
  object HandType {
    given Order[HandType] = Order.by(-_.ordinal)
    given Ordering[HandType] = summon[Order[HandType]].toOrdering

    def determine(hand: Vector[Card]): HandType = {
      val grouped = hand.groupBy(identity).view.mapValues(_.size).iterator
      val sorted = grouped.toVector.sortBy(-_._2)
      sorted match {
        case (_, 5) +: _ => FiveOfAKind
        case (_, 4) +: _ => FourOfAKind
        case (_, 3) +: (_, 2) +: _ => FullHouse
        case (_, 3) +: _ => ThreeOfAKind
        case (_, 2) +: (_, 2) +: _ => TwoPair
        case (_, 2) +: _ => OnePair
        case _ => HighCard
      }
    }

    /** As [[determine]] but [[Card.Jack]] is now a wildcard, matching any other card type. */
    def determinePart2(hand: Vector[Card]): HandType = {
      val grouped = hand.groupBy(identity).view.mapValues(_.size).iterator
      val sorted = grouped.toVector.sortBy { case (card, count) =>
        if card == Card.Jack then Int.MinValue else -count
      }

      sorted match {
        case
          (Card.Jack, 4) +: (_, 1) +: _
          | (Card.Jack, 3) +: (_, 2) +: _
          | (Card.Jack, 2) +: (_, 3) +: _
          | (Card.Jack, 1) +: (_, 4) +: _
          | (_, 5) +: _
        => FiveOfAKind
        case
          (Card.Jack, 3) +: (_, 1) +: _
          | (Card.Jack, 2) +: (_, 2) +: _
          | (Card.Jack, 1) +: (_, 3) +: _
          | (_, 4) +: _
        => FourOfAKind
        case
          (Card.Jack, 2) +: (_, 2) +: (_, 1) +: _
          | (Card.Jack, 1) +: (_, 2) +: (_, 2) +: _
          | (_, 3) +: (_, 2) +: _
        => FullHouse
        case
          (Card.Jack, 2) +: (_, 1) +: _
          | (Card.Jack, 1) +: (_, 2) +: _
          | (_, 3) +: _
        => ThreeOfAKind
        case
          (Card.Jack, 1) +: (_, 2) +: (_, 1) +: _
          | (_, 2) +: (_, 2) +: _
          => TwoPair
        case
          (Card.Jack, 1) +: (_, 1) +: _
          | (_, 2) +: _
        => OnePair
        case _ => HighCard
      }
    }

    def compareOneByOne(hand1: Vector[Card], hand2: Vector[Card])(using Order[Card]): Comparison = {
      hand1.iterator.zip(hand2)
        .map { case (c1, c2) => c1 comparison c2 }
        .find(_ != Comparison.EqualTo)
        .getOrElse(Comparison.EqualTo)
    }
  }

  case class Hand(cards: Vector[Card]) {
    lazy val handType: HandType = HandType.determine(cards)
    lazy val handTypePart2: HandType = HandType.determinePart2(cards)

    override def toString = s"Hand(${cards.mkString("")}, part1=$handType, part2=$handTypePart2)"
  }
  object Hand {
    given ordering: Ordering[Hand] = Ordering.by((_: Hand).handType).orElse(Ordering.fromLessThan { (h1, h2) =>
      HandType.compareOneByOne(h1.cards, h2.cards) == Comparison.LessThan
    })

    val orderingPart2: Ordering[Hand] = Ordering.by((_: Hand).handTypePart2).orElse(Ordering.fromLessThan { (h1, h2) =>
      HandType.compareOneByOne(h1.cards, h2.cards)(using Card.part2IndividualOrder) == Comparison.LessThan
    })
  }

  case class Entry(hand: Hand, bid: Int)

  def parse(data: Vector[String]): Vector[Entry] = {
    data.map { line =>
      val (cardsStr, bidStr) = (line.take(5), line.drop(6))
      val hand = Hand(cardsStr.iterator.map(Card.parse).toVector)
      Entry(hand, bidStr.toInt)
    }
  }

  case class Ranked[A](value: A, rank: Int)

  def run(data: Vector[String])(using Ordering[Hand]): String = {
    val entries = parse(data)
    val result = entries.sortBy(_.hand).iterator.zipWithIndex.map { case (entry, i) =>
      val rank = i + 1
      Ranked(value = entry, rank = rank)
    }.map(ranked => ranked.value.bid * ranked.rank).sum
    result.toString
  }

  def run1(data: Vector[String]): String = run(data)(using Hand.ordering)

  def run2(data: Vector[String]): String = run(data)(using Hand.orderingPart2)
}

object _7_1_Test extends Problem(7, InputMode.Test(1), Solution7.run1)
object _7_1_Normal extends Problem(7, InputMode.Normal, Solution7.run1)

object _7_2_Test extends Problem(7, InputMode.Test(1), Solution7.run2)
object _7_2_Normal extends Problem(7, InputMode.Normal, Solution7.run2)