import org.testng.annotations.Test
import solution.Day07
import kotlin.test.assertEquals

class Day07Test {

    @Test
    fun testDay07_part1() {
        val input = Util.getResourceAsText("examples/07.txt")

        val day = Day07()
        val result = day.solvePart1(input)

        assertEquals("95437", result)
    }

    @Test
    fun testDay07_part2() {
        val input = Util.getResourceAsText("examples/07.txt")

        val day = Day07()
        val result = day.solvePart2(input)

        assertEquals(0, result)
    }
}
