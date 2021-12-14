using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace Runity {
    public class RunityMono : MonoBehaviour {
        private RunityBackend m_runity;
        [SerializeField] private ScriptablePrefabs m_scriptablePrefabs;
        [SerializeField] private string m_gameLibraryName;
        [SerializeField] private Level m_unityLoggerLevel = Level.Info;
        [SerializeField] private Level m_writerLoggerLevel = Level.Debug;
        // Start is called before the first frame update
        void Start() {
            m_runity = new RunityBackend(
                m_gameLibraryName,
                m_scriptablePrefabs,
                m_unityLoggerLevel,
                m_writerLoggerLevel
            );
        }
        void Update() {
            m_runity.Update();
        }
    }
}