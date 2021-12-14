# runity
![](runity_logo.png)

Highly experimental Rust implementation for Unity.

Utilizing Bevy's entity component system, we can write code like any other Bevy project,
but use Unity as the runtime.

### Runity features:
This is Runity currently capable of controlling from Rust to Unity
1. spawn prefab
2. spawn prefab as a child of another game object
3. destroy prefab
4. process collision events (very barebones)
5. modify transform position
6. modify TextMesh text

### how to build
  I currently don't provide a unity demo project, but I have some example code in rust/demo_game.
  If you follow these instructions the game code will expect you to setup 2 ScriptablePrefabs called 'monkey' and 'banana'
  - Unity 
  1. add the 'Runity' unity plugin to your unity project. (You can copy the cs files from the unity folder!)
  2. in your scene, add a RunityMono component and assign the name of your 'game.dll' or what your Rust game project will be called.
  - Rust
  1. Fork or clone this repository
  2. in the rust folder, we have runity/shared/demo_game, build runity and demo_game
  3. move 'runity.dll' into the unity project, where you placed the runity plugin (unity_project/assets/plugins/runity/runity.dll)
  4. move 'demo_game.dll' into the root of the unity project (unity_project/)

### How it works
  ##### Runity Unity plugin
    Responsible for loading the runity.dll. All C# & Rust communication goes through these scripts.

  ##### runity.dll
    Handles all C# communication, responsible for the Bevy world. Loads and communicates with the game code 'game.dll'.
    Bevy systems are setup to sync the Rust side into C#.
    Because of the 'ugly' FFI nature of code I seperated the game code with the communication code
    
  ##### game.dll 'renamable'
    Game specific code
    

### Areas of improvement
Some ideas I got that needs to be improved
* Instead of holding the game.dll inside the unity project, it should be kept outside so we don't have to close Unity every recompile
* asking Unity for Input is a slow operation because the system always has to be singlethreaded. Maybe inputs should be dumped every frame into rust, allowing for multithreaded systems accessing input
* Spawning gameobjects within a hiearchy of children/parents doesn’t work. They must be sent to Unity in a correct order, which they don’t atm.
* the game.dll and runity.dll has to be compiled withing the same project (I believe). Would be nice if it could be a seperate project.
* RectTransform implementation is hardcoded in C# (don’t look very scary)
* Figure out how to actually process collsion in a neat way instead of having to access the whole bevy world in the system, causing very ugly and inefficent code.
* Hashing of component in C# currently uses a hardcoded identifier. Adding new components may cause unsound logic. (Done in case we need Rust to access Unity Component values like syncing transform before the Update, which we currently don’t do)
