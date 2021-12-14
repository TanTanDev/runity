using System.Collections.Generic; 

namespace Runity {
    // Used to access parts of a Unity GameObject, from rust
    public struct RustyGameObject {
        public UnityEngine.GameObject GameObject;
        public Dictionary<int, UnityEngine.Component> HashedComponents; 

        public RustyGameObject(UnityEngine.GameObject a_gameObject) {
            GameObject = a_gameObject;
            HashedComponents = new Dictionary<int, UnityEngine.Component>();
        }

        public T GetComponentFromId<T>(int a_componentId)
            where T: UnityEngine.Component
        {
            if(HashedComponents.ContainsKey(a_componentId))
                return HashedComponents[a_componentId] as T;
            else {
                var component = GameObject.GetComponent<T>();
                HashedComponents[a_componentId] = component;
                return HashedComponents[a_componentId] as T;
            }
        }
    }
}