package logic.wfc

import entity.board.Board
import common.constant.Settings
import kotlin.random.Random

internal class WaveFunctionCollapse(val board: Board) {
    val boardConstraints: BoardConstraints = BoardConstraints(board)

    fun rankPossibleStates(): List<Int> {
        return findAllPossibleStates().map { it.allPossibleWords() }
    }

    fun selectPossibleLocation(): BoardCellState {
        val possibleLocations = findSuperpositions()

        if (possibleLocations.size == 1) return possibleLocations[0]
        return possibleLocations[Random.nextInt(0, possibleLocations.size - 1)]
    }

    // This function searches for superposition on board
    private fun findSuperpositions(): List<BoardCellState> {
        val allStates = findAllPossibleStates()
        val max = allStates.maxOf { it.allPossibleWords() }
        val min = allStates
                    .map { it.allPossibleWords() }
                    .filter {  it > 0 }
                    .minOf { it }

        val possibleLocations = allStates.filter { wfcp ->
            (wfcp.allPossibleWords() == min)
        }

        println(possibleLocations.map { it.row * Settings.COLS + it.col })

        return possibleLocations

    }

    // This is looking for all possible states on board
    private fun findAllPossibleStates(): List<BoardCellState> {
        // create indexed board checking all possible states
        val indexedBoard = board.board.mapIndexed { idx, s ->
            if (idx / Settings.COLS == 4 && idx % Settings.COLS == 6)
                println()
            BoardCellState(
                boardConstraints,
                // Current row
                idx / Settings.COLS,
                // Current col
                idx % Settings.COLS
            )
        }

        return indexedBoard
    }
}