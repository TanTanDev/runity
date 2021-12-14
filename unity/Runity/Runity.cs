using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;
using System.Runtime.InteropServices;

namespace Runity {
    public class RunityBackend {
        private Api m_api; 
        private Logging m_logging; 
        private World m_world; 

        public RunityBackend(
            string a_gameLibName,
            ScriptablePrefabs a_prefabBindings,
            Level a_unityLoggerLevel,
            Level a_writerLoggerLevel
        ) {
            m_logging = new Logging();
            m_world = new World(a_prefabBindings);
            m_api = new Api(a_gameLibName, a_unityLoggerLevel, a_writerLoggerLevel);
        }

        public void Update() {
            // before scripts run, send them latest collision events
            m_world.DownloadEntityCollisionEvents();
            m_api.Update();
        }
    }

    public class Api {
        public Api(string a_gameLibName, Level a_unityLoggerLevel, Level a_writerLoggerLevel) {
            api_init(a_unityLoggerLevel, a_writerLoggerLevel);
            api_init_game_lib(a_gameLibName);
        }

        public void Update() {
            Time time;
            time.delta_time = UnityEngine.Time.deltaTime;
            api_update(time);
        }

        [DllImport("runity")] public static extern void api_init(Level a_unityLoggerLevel, Level a_writerLoggLevel);
        [DllImport("runity")] public static extern void api_update(Time a_time);

        // game specific rust dll
        [DllImport("runity")] public static extern void api_init_game_lib(string a_gameLibName);
    }
}