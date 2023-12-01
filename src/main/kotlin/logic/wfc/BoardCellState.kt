package logic.wfc

import entity.board.Direction
import entity.board.Word
import kotlin.random.Random

internal class BoardCellState(boardConstraints: BoardConstraints, val row: Int, val col: Int) {
    private val directionalMap: Map<Direction, Set<Word>> = mapOf(
        Pair(Direction.LTR, boardConstraints.findWordsFromPositionLTR(row, col)),
        Pair(Direction.RTL, boardConstraints.findWordsFromPositionRTL(row, col)),
        Pair(Direction.TTB, boardConstraints.findWordsFromPositionTTB(row, col)),
        Pair(Direction.BTT, boardConstraints.findWordsFromPositionBTT(row, col))
    )

    fun allPossibleWords() = directionalMap.entries.fold(0) { sum, direction ->
        sum + direction.value.size
    }

    fun getWord(): Pair<Direction, Word> {
        val direction = determineDirection()
        val word = determineWord(directionalMap[direction]!!)

        return Pair(direction, word)
    }

    fun getDirection(): Pair<Direction, Set<Word>> {
        val idealDirection = determineDirection()
        return Pair(idealDirection, directionalMap.getOrDefault(idealDirection, emptySet()))
    }

    private fun determineDirection(): Direction {
        val max = directionalMap.maxOf { it.value.size }
        val idealDirections = directionalMap.filter { it.value.size == max }.map { it.key }

        if (idealDirections.size == 1) return idealDirections[0]
        return idealDirections[Random.nextInt(0, idealDirections.size - 1)]
    }

    private fun determineWord(dictionary: Set<Word>): Word =
        if (dictionary.size > 1 )
            dictionary.random()
        else dictionary.first()

}