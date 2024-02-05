最新状态（2024.2.5 14:30）：通过了一些对拍。
Latest status (14:30 05/02/2024): The programme has suceeded in some batched tests.

**警告：本程序可能有潜在的 bug。**

针对 THUAC 2024 游戏 Generals Impact 写的民间自用 rust 播放器 & sdk。使用造成的风险和后果自负。如果您使用本代码在比赛中获奖，建议您请作者吃饭。

原〇游戏的 AI，就是要用编程语言的原〇写！

**Warning: This program may have undiscovered bugs.**

Homemade rust player & sdk for Generals Impact, the game of THUAC 2024. Use at your own risk! If you get a award in the game by using this code, treating the author is recommended.

The AI for a Gen.* game, should be undoubtfully written in the Gen.* of programming language!

### 作为播放器使用 Use as a player

在 `player.rs` 里面实现了播放器的功能。如果你想把本程序作为播放器使用，只需要直接编译程序，然后将回放文件名称改为 map.json 并放在运行目录下。如果安装了 cargo 之类的，并且在 Linux 环境下，应该可以直接运行 ./run.sh 来编译运行。游戏结束后代码会 panic（因为懒得写判断了），这是正常的，无需惊慌。

In `player.rs` the function of player is implemented. If you want to use this program as a player, just simply compile the program, and rename the replay file into `map.json` and place the replay file under the directory of execution. After the ending of the game the program will panic (because I'm too lazy to check it), which is normal and you don't need to panic.

在作为播放器使用时，按回车播放下一步。输入`j 数字`后按回车跳转到对应回合的开头。

When using as a player, press enter to step. Enter `j number` then press enter to jump to the starting of the corresponding round.

### 作为 sdk 使用 Use as sdk

可以参考 `operation.rs`，其中实现了 `send_op` 函数，可以发送一系列操作到标准输出。

You may take a look at `opertion.rs`, where the function `send_op` is implemented, which can send a series of operations to stdout.

### 称呼约定 Convention of Names

为了减少认知负担和打字负担，游戏里的称呼可能有所改动，通常以简单的单词作为命名。约定如下：

For reducing cognitive and typing burden, the names in the game might be changed; usually we choose simple and short words for naming. The conventions are stated below:

#### 游戏单位态度 Attitude of Units

我们不按照玩家先后手区分地图上的单位阵营，而是根据当前具有操作权的玩家，分为友好（`Attitude::Friendly`）、中立（`Attitude::Neutral`）、敌对（`Attitude::Hostile`）三种。当一个玩家结束其操作时，调用 `flip` 函数来处理阵营翻转。我相信这样的设计有利于搜索和逻辑判断。

We don't differentiate the factions by first/second mover; instead, we have 3 factions depending on the current operating player: `Attitude::Friendly`, `Attitude::Neutral` and `Attitude::Hostile`. When a player finished their turn, we call the `flip` function to deal with flipping of factions. I believe that such designation is good for searching and logical checking.

#### 将领类型与等级 Types and Levels of Generals

主将 Main General: `GeneralType::Main`
副将 Sub General: `GeneralType::Sub`
油田在代码中称作矿 Oil wells are called mines in the code: `GeneralType::Mine`

等级 levels：`AttrType::Prod`（产量增加 increasing of production）、`AttrType::Def`（防御增加 increasing of defence）、`Attr::Spd`（速度增加 increasing of speed）。

#### 普通移动 Normal Troop Movement

部队移动在代码中称作 `march`。将军移动在代码中称作 `shift`。

Moving of troops is called `march` in the code. Moving of generals is called `shift` in the code.

#### 技能 General Skills

五种技能的名称在代码中分别为 `dash`（突袭）、`kill`（击破）、`atk`（统率）、`def`（坚守）、`magic`（弱化），名字简单并且和效果挂钩。最后三类在代码中统称 `buff`。

Five skills are called `dash`, `kill`, `atk`, `def` and `magic` in the code. Names are simple and related to the effect. The last 3 skills are categorized as `buff` in the code.

#### 超级武器 Super Weapons

四个超级武器在代码中分别为 `SWType::Nuclear`（核弹）、`SWType::Boost`（攻击强化）、`SWType::Teleport`（超时空传送）、`SWType::Freeze`（时间停止）。同时，冷却中记作 `SWType::Pending`。

Four super weapons are `SWType::Nuclear` (Nuclear Bomb), `SWType::Boost` (Attack Boost), `SWType::Teleport` (Teleport), `SWType::Freeze` (Freeze Units) in the code. When on CD, the type is `SWType::Pending`.

#### 科技 Technology

军队行动力 Maneuver of troops：`TechType::Motor`（摩托化 motorization）
免疫沼泽 Immunity of swamp：`TechType::Raft`（筏子 raft）
免疫流沙 Immunity of quicksand: `TechType::Track`（履带 track）
解锁超级武器 Unlock super weapons: `TechType::Relativity`（相对论 relativity...?）

### 重要的全局函数 Important global functions

#### `read_init`


### 游戏状态类 Class for the State of the Game

所有游戏状态需要的信息都在 `gamestate/definition.rs` 的 `GameState` 类中被保存。详见代码。

All information for the game state are in struct `GameState` in `gamestate/definition.rs`. See the code for details.

它有一些重要的方法：

It has some important methods:

#### `print`

实现在 `colorize.rs` 的 `GameState::print(&self)` 函数可以将一个游戏状态输出到终端。虽然不是很好看，但是包含了需要的大部分信息：

The function `GameState::print(&self)`, implemented in `colorize.rs`, can print a gamestate to the therminal. Its' not beautiful but contains most information needed.

- 一个格子的信息形如 `CXX`，其中 `C` 是一个字母或者符号，`XX` 是数字。
- 信息的文字颜色表示阵营。绿色是友军，红色是敌军，黑色是中立。
- 信息的背景颜色表示地形。白色是平地，浅黄色是流沙，浅蓝色是沼泽。
- `C` 这个字母或者符号表示该格子的将领。如果格子的将军是油田，将领使用符号表示，同时格子信息有下划线。如果该格子的将军是将领，使用字母表示。
- `XX` 是数字，表示在该格子的兵力。

- The information in one cell is writen as `CXX`, where `C` is a letter or symbol and `XX` is a number.
- Text color shows the faction. Green for friendly, red for hostile and black for neutral.
- Background color shows the terrain. White for plain, light yellow for quicksand and light blue for swamp.
- `C` stands for the general in the cell. Symbols w/ underlines are for oil wells and letters are for normal generals.
- `XX` for number of units in the cell.

如果你的终端不支持精细的颜色（不会吧？），或者你觉得配色因为各种原因难以分辨/太丑陋了，可以考虑自行修改 `colorize.rs` 的内容。

If your terminal does not support the fine colors, or you think the colors are hard to distinguish or too ugly, you can modify the content in `colorize.rs` by yourself.

#### `apply_op`

对游戏状态应用一个操作。

Apply an operation to the gamestate.

#### `flip`

在一个玩家结束其操作以后，`flip` 函数将会切换视角——即翻转阵营状态。如果是后手玩家结束操作，还会进行结算。

After a player finished their operation, the `flip` function will change the view aka flipping the faction status. If it is the second mover ending their turn, the end-turn calculation will also be executed.

#### `skills.rs` 里的所有函数 | All functions in `skill.rs`

这些函数掌管检查和执行普通移动、技能和超级武器。

These functions are for normal movements, skills and super weapons.

### 后记

更多内容参见代码。欢迎提疑修。

For more contents see the code. Issues welcomed.