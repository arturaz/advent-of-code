package aoc

import com.google.common.collect.{BoundType, ImmutableRangeMap, RangeMap, Range as GRange}
import com.softwaremill.quicklens.*
import cats.syntax.all.*
import cats.effect.syntax.all.*

import scala.collection.parallel.CollectionConverters.*
import scala.collection.immutable.NumericRange

object Solution5 {
  case class MappingResult(value: Long, valuesTillMappingEnd: Long)

  case class MappingRange(destinationRangeStart: Long, sourceRangeStart: Long, rangeLength: Long) {
    inline def destinationRangeEnd: Long = destinationRangeStart + rangeLength
    inline def sourceRangeEnd: Long = sourceRangeStart + rangeLength

    override def toString =
      s"MappingRange[$sourceRangeStart..${sourceRangeEnd - 1} -> $destinationRangeStart..${destinationRangeEnd - 1}]"

    inline def getOrThrow(index: Long): MappingResult = {
      if (index >= sourceRangeStart && index < sourceRangeEnd) {
        val offset = index - sourceRangeStart
        val value = destinationRangeStart + offset
        val valuesTillMappingEnd = rangeLength - offset - 1
        MappingResult(value, valuesTillMappingEnd)
      } else {
        throw new Exception(s"Index $index is out of range $this")
      }
    }
  }
  object MappingRange {
    def parse(line: String): MappingRange = {
      // 50 98 2
      val Array(destinationRangeStart, sourceRangeStart, rangeLength) = line.split(" ").map(_.toLong)
      MappingRange(destinationRangeStart, sourceRangeStart, rangeLength)
    }
  }

  //noinspection UnstableApiUsage
  case class Mappings(mappings: Vector[MappingRange]) {
    lazy val rangeMap: RangeMap[java.lang.Long, MappingRange] = {
      val b = ImmutableRangeMap.builder[java.lang.Long, MappingRange]()
      mappings.foreach { mapping =>
        b.put(GRange.closedOpen(mapping.sourceRangeStart, mapping.sourceRangeEnd), mapping)
      }
      b.build()
    }

    /**
     * A map of unmapped zones.
     *
     * So if [[rangeMap]] is {[-10..10] -> v1, [20..30] -> v2} then [[unmappedMap]] is
     * {[Long.MinValue..-11] -> v1, [11..19] -> v2}.
     * */
    lazy val unmappedMap: RangeMap[java.lang.Long, MappingRange] = {
      var begin = Long.MinValue
      val b = ImmutableRangeMap.builder[java.lang.Long, MappingRange]()
      rangeMap.asMapOfRanges().forEach { (range, mapping) =>
        if (range.lowerEndpoint() > begin) {
          b.put(GRange.closedOpen(begin, range.lowerEndpoint()), mapping)
        }
        begin = range.upperEndpoint() + 1
      }
      b.build()
    }

    def apply(index: Long): MappingResult = {
      rangeMap.get(index) match {
        case null =>
          val toSkip = unmappedMap.get(index) match {
            case null => Long.MaxValue
            case mapping => mapping.sourceRangeStart - index - 1
          }
          MappingResult(index, toSkip)
        case mapping => mapping.getOrThrow(index)
      }
    }

    def :+(mapping: MappingRange): Mappings = this.modify(_.mappings).using(_ :+ mapping)
  }
  object Mappings {
    def empty: Mappings = Mappings(Vector.empty)
  }

  case class Data(
    seeds: Vector[NumericRange.Exclusive[Long]],
    seedToSoilMap: Mappings,
    soilToFertilizerMap: Mappings,
    fertilizerToWaterMap: Mappings,
    waterToLightMap: Mappings,
    lightToTemperatureMap: Mappings,
    temperatureToHumidityMap: Mappings,
    humidityToLocationMap: Mappings
  ) {
    override def toString =
      s"""Data[
         |  seeds: $seeds
         |  seedToSoilMap: $seedToSoilMap
         |  soilToFertilizerMap: $soilToFertilizerMap
         |  fertilizerToWaterMap: $fertilizerToWaterMap
         |  waterToLightMap: $waterToLightMap
         |  lightToTemperatureMap: $lightToTemperatureMap
         |  temperatureToHumidityMap: $temperatureToHumidityMap
         |  humidityToLocationMap: $humidityToLocationMap
         |]""".stripMargin

    def resolveSeedLocation(seed: Long): MappingResult = {
      val soil = seedToSoilMap(seed)
      val fertilizer = soilToFertilizerMap(soil.value)
      val water = fertilizerToWaterMap(fertilizer.value)
      val light = waterToLightMap(water.value)
      val temperature = lightToTemperatureMap(light.value)
      val humidity = temperatureToHumidityMap(temperature.value)
      val location = humidityToLocationMap(humidity.value)

      val canBeSkipped =
        soil.valuesTillMappingEnd min fertilizer.valuesTillMappingEnd min water.valuesTillMappingEnd min
        light.valuesTillMappingEnd min temperature.valuesTillMappingEnd min humidity.valuesTillMappingEnd min
        location.valuesTillMappingEnd

//      println(
//        s"Seed $seed: soil $soil, fertilizer $fertilizer, water $water, light $light, temperature $temperature, " +
//        s"humidity $humidity, location $location, canBeSkipped $canBeSkipped"
//      )

      MappingResult(location.value, canBeSkipped)
    }
  }
  object Data {
    def empty: Data = apply(
      Vector.empty,
      Mappings.empty, Mappings.empty, Mappings.empty, Mappings.empty, Mappings.empty, Mappings.empty, Mappings.empty
    )
  }

  enum DataParserState {
    case Seeds(isRange: Boolean)
    case SeedToSoil
    case SoilToFertilizer
    case FertilizerToWater
    case WaterToLight
    case LightToTemperature
    case TemperatureToHumidity
    case HumidityToLocation
  }
  case class DataParser(state: DataParserState, data: Data) {
    def input(line: String): DataParser = {
      def modMappings(mappings: Mappings): Mappings = mappings :+ MappingRange.parse(line)

      def parseLine() = state match {
        case DataParserState.Seeds(isRange) =>
          // seeds: 79 14 55 13
          if (line.startsWith("seeds: ")) {
            val seeds = line.split(" ").drop(1).map(_.toLong)
            val ranges =
              if (isRange) seeds.grouped(2).map { case Array(start, length) => start until (start + length) }
              else seeds.iterator.map(seed => seed until (seed + 1))
            this.modify(_.data.seeds).setTo(ranges.toVector)
          }
          else throw new Exception(s"Unexpected line: $line")
        case DataParserState.SeedToSoil => this.modify(_.data.seedToSoilMap).using(modMappings)
        case DataParserState.SoilToFertilizer => this.modify(_.data.soilToFertilizerMap).using(modMappings)
        case DataParserState.FertilizerToWater => this.modify(_.data.fertilizerToWaterMap).using(modMappings)
        case DataParserState.WaterToLight => this.modify(_.data.waterToLightMap).using(modMappings)
        case DataParserState.LightToTemperature => this.modify(_.data.lightToTemperatureMap).using(modMappings)
        case DataParserState.TemperatureToHumidity => this.modify(_.data.temperatureToHumidityMap).using(modMappings)
        case DataParserState.HumidityToLocation => this.modify(_.data.humidityToLocationMap).using(modMappings)
      }

      line match {
        case "" => this
        case "seed-to-soil map:" => this.modify(_.state).setTo(DataParserState.SeedToSoil)
        case "soil-to-fertilizer map:" => this.modify(_.state).setTo(DataParserState.SoilToFertilizer)
        case "fertilizer-to-water map:" => this.modify(_.state).setTo(DataParserState.FertilizerToWater)
        case "water-to-light map:" => this.modify(_.state).setTo(DataParserState.WaterToLight)
        case "light-to-temperature map:" => this.modify(_.state).setTo(DataParserState.LightToTemperature)
        case "temperature-to-humidity map:" => this.modify(_.state).setTo(DataParserState.TemperatureToHumidity)
        case "humidity-to-location map:" => this.modify(_.state).setTo(DataParserState.HumidityToLocation)
        case _ => parseLine()
      }
    }
  }

  def run(data: Vector[String], isRange: Boolean): String = {
    val parsed = data.foldLeft(DataParser(DataParserState.Seeds(isRange), Data.empty))(_.input(_)).data
    println(parsed)

//    val result = parsed.seeds.par.map(range => range.iterator.map(parsed.resolveSeedLocation(_).value).min).min
    val result = parsed.seeds.par.map { range =>
      var current = range.start
      var min = Long.MaxValue
      while (current < range.end) {
        val result = parsed.resolveSeedLocation(current)
        min = min.min(result.value)
        current += 1 + result.valuesTillMappingEnd
      }
      min
    }.min
    result.toString
  }

  def run1(data: Vector[String]): String = run(data, isRange = false)
  def run2(data: Vector[String]): String = run(data, isRange = true)
}

object _5_1_Test extends Problem(5, InputMode.Test(1), Solution5.run1)
object _5_1_Normal extends Problem(5, InputMode.Normal, Solution5.run1)

object _5_2_Test extends Problem(5, InputMode.Test(1), Solution5.run2)
object _5_2_Normal extends Problem(5, InputMode.Normal, Solution5.run2)