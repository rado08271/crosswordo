package game

import entity.board.Board
import entity.board.GameState
import entity.player.Player
import entity.board.Word
import logic.wfc.BoardPropagator

class GamePlay(val board: Board, val player: Player, val solution: Word) {
    val boardPropagator: BoardPropagator = BoardPropagator(board)
    var gameState: GameState = GameState.INIT

    fun initGame() {
        if (gameState == GameState.INIT) {
//            board.initNotRandom(solution)
            board.randomAssign(solution)
            gameState = GameState.STARTED
        }
    }

    fun gameTick() {
        if (gameState == GameState.STARTED) {
            // TODO do some logic
            gameState = GameState.PLAYING
        }

        boardPropagator.fillSingleWord()
        if (boardPropagator.isBoardPropagated())
            if (board.board.contains(" ")) {
                gameState = GameState.FAILED
            } else {
                gameState = GameState.FINISHED
            }
    }


}