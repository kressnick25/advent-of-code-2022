import org.testng.annotations.Test
import solution.Day05
import kotlin.test.assertEquals

class Day05Test {

    @Test
    fun testDay05_part1() {
        val input = Util.getResourceAsText("examples/05.txt")

        val day = Day05()
        val result = day.solvePart1(input)

        println("Result: $result")
        assertEquals("CMZ", result)
    }

    @Test
    fun testDay05_part2() {
        val input = Util.getResourceAsText("examples/05.txt")

        val day = Day05()
        val result = day.solvePart2(input)

        assertEquals("MCD", result)
    }
}
