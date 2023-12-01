package game

import common.constant.Settings
import entity.board.Board
import entity.Dictionary
import entity.board.GameState
import entity.player.Player
import entity.board.Word
import logic.wfc.WaveFunctionCollapse
import ui.Console

class Game {
    val gameUi: Console = Console()

    lateinit var gamePlay: GamePlay

    lateinit var board: Board
    lateinit var solution: Word
    lateinit var player: Player

    fun initGame() {
        board = Board()
        solution = Word("automat", 0)
        player = Player()

        gamePlay = GamePlay(board, player, solution)

        println("erererer")
        gamePlay.initGame()
        println("orororo")
    }

    fun playGame() {
        val start = System.currentTimeMillis()
        gamePlay.gameTick()

        while (gamePlay.gameState == GameState.PLAYING) {
//            gameUi.drawBoard(board)
//            printBoardStates()
            gamePlay.gameTick()
        }
        gameUi.drawBoard(board)

        println()
        board.dictionary.usedWords.forEach {
            println("-\t${it.word}")
        }

        val end = System.currentTimeMillis()
        println("Game finished in ${end - start}[ms] as ${gamePlay.gameState}")
    }

    fun printBoardStates() {
        val board = WaveFunctionCollapse(board).rankPossibleStates()
        board.forEachIndexed { idx, state ->
            if (idx % Settings.COLS == 0) println()
            print("\t${state}")
        }
        println()
    }

//    fun repeat() {
//        wafeFunctionCollapse.waveFunctionCollapse().forEachIndexed { index, s ->
//            if (index % Settings.COLS == 0) println()
//            print("|\t${s}\t|")
//        }
//
//        val position = wafeFunctionCollapse.selectPerfectPosition()
//        val directionalDictionary = position.getDirection()
//        val word =
//            if (directionalDictionary.second.size == 1 )
//                directionalDictionary.second[Random.nextInt(0, directionalDictionary.second.size)]
//            else directionalDictionary.second[0]
//
//        boardPropagator.propagateWordToBoard(position.col, position.row, directionalDictionary.first, word)
//    }
}