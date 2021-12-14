using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace Runity {
    [System.Serializable]
    public struct PrefabBinding {
        public GameObject Prefab;
        public string RustName;
    }

    [CreateAssetMenu(fileName = "Runity_ScriptablePrefabs", menuName ="Runity/ScriptablePrefabs")]
    public class ScriptablePrefabs : ScriptableObject {
        public PrefabBinding[] PrefabBindings; 
    }
}