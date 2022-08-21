# runity
![](runity_logo.png)

Highly experimental Rust implementation for Unity.

Utilizing Bevy's entity component system, we can write code like any other Bevy project,
but use Unity as the runtime.

I documented the creation of this project on my youtube channel called Tantan:
https://youtu.be/L7M_vbo1N2g 

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
  1. add the 'Runity' unity plugin to your unity project. (You can copy the Runity folder into you unity plugins folder "unity/Runity")
  2. in your scene, add a RunityMono component and assign the name of your 'demo_game.dll' or what your Rust game project will be called.
  - Rust
  1. Fork or clone this repository
  2. in the rust folder, we have runity/shared/demo_game, build runity and demo_game
  3. move 'runity.dll' into the unity project, where you placed the runity plugin (unity_project/assets/plugins/runity/runity.dll)
  4. move 'demo_game.dll' into the root of the unity project (unity_project/)

### What is up with 'game' folder and 'demo_game'?
the game folder was the spike game for the video.
There you can find some example code on how I handled collision and text as you can see in the video I made about this project.
demo_game was the code for the small monkey banana project I also showed in my video.

### How it works
  ##### unity/Runity, C# plugin code
    Responsible for loading the runity.dll. All C# & Rust communication goes through these scripts.

  ##### rust/runity (dll project)
    Handles all C# communication, responsible for the Bevy world. Loads and communicates with the game code 'game.dll'.
    Bevy systems are setup to sync the Rust side into C#.
    Because of the 'ugly' FFI nature of code I separated the game code with the communication code
    
  ##### rust/game or rust/demo_game (dll projects)
    Example game projects, all gameplay code is written in such a project, communicates with runity.dll
    
  ##### shared
    this library is used by both runity and game projects
    

### Areas of improvement
Some ideas I got that needs to be improved
* Instead of holding the game.dll inside the unity project, it should be kept outside so we don't have to close Unity every recompile
* asking Unity for Input is a slow operation because the system always has to be singlethreaded. Maybe inputs should be dumped every frame into rust, allowing for multithreaded systems accessing input
* Spawning gameobjects within a hierarchy of children/parents doesn’t work. They must be sent to Unity in a correct order, which they don’t atm.
* the game.dll and runity.dll has to be compiled withing the same project (I believe). Would be nice if it could be a separate project.
* RectTransform implementation is hardcoded in C# (don’t look very scary)
* Figure out how to actually process collision in a neat way instead of having to access the whole bevy world in the system, causing very ugly and inefficient code.
* Hashing of component in C# currently uses a hardcoded identifier. Adding new components may cause unsound logic. (Done in case we need Rust to access Unity Component values like syncing transform before the Update, which we currently don’t do)
