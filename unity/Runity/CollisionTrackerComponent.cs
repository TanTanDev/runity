using UnityEngine;
using System.Collections.Generic;
using System;
using System.Runtime.InteropServices;

namespace Runity {
    [StructLayout(LayoutKind.Sequential)]
    public struct CollisionEvent {
        public UInt64 OwnerEntityId;
        public UInt64 OtherEntityId;
        public CollisionType CollisionType;
        public CollisionEvent(UInt64 a_ownerEntityId, UInt64 a_otherEntityId, CollisionType a_collisionType) {
            OwnerEntityId = a_ownerEntityId;
            OtherEntityId = a_otherEntityId;
            CollisionType = a_collisionType;
        }

    }

    public enum CollisionType {
        OnCollisionEnter,
        OnCollisionExit,
        OnCollisionStay,
        OnTriggerEnter,
        OnTriggerExit,
        OnTriggerStay,
    }

    public class CollisionTrackerComponent : MonoBehaviour {
        public Queue<CollisionEvent> CollisionEvents;
        public UInt64 OwnerEntityIdBits;

        void OnCollisionEnter(Collision a_collision) {
            var entityIdentifier = a_collision.gameObject.GetComponent<RustEntityComponent>();
            if(entityIdentifier != null)
                CollisionEvents.Enqueue(new CollisionEvent(OwnerEntityIdBits, entityIdentifier.IdentifierBits, CollisionType.OnCollisionEnter));
        }
        void OnCollisionExit(Collision a_collision) {
            var entityIdentifier = a_collision.gameObject.GetComponent<RustEntityComponent>();
            if(entityIdentifier != null)
                CollisionEvents.Enqueue(new CollisionEvent(OwnerEntityIdBits, entityIdentifier.IdentifierBits, CollisionType.OnCollisionExit));
        }
        void OnCollisionStay(Collision a_collision) {
            var entityIdentifier = a_collision.gameObject.GetComponent<RustEntityComponent>();
            if(entityIdentifier != null)
                CollisionEvents.Enqueue(new CollisionEvent(OwnerEntityIdBits, entityIdentifier.IdentifierBits, CollisionType.OnCollisionStay));
        }
        void OnTriggerEnter(Collider a_collider) {
            var entityIdentifier = a_collider.gameObject.GetComponent<RustEntityComponent>();
            if(entityIdentifier != null)
                CollisionEvents.Enqueue(new CollisionEvent(OwnerEntityIdBits, entityIdentifier.IdentifierBits, CollisionType.OnTriggerEnter));
        }
        void OnTriggerExit(Collider a_collider) {
            var entityIdentifier = a_collider.gameObject.GetComponent<RustEntityComponent>();
            if(entityIdentifier != null)
                CollisionEvents.Enqueue(new CollisionEvent(OwnerEntityIdBits, entityIdentifier.IdentifierBits, CollisionType.OnTriggerExit));
        }
        void OnTriggerStay(Collider a_collider) {
            var entityIdentifier = a_collider.gameObject.GetComponent<RustEntityComponent>();
            if(entityIdentifier != null)
                CollisionEvents.Enqueue(new CollisionEvent(OwnerEntityIdBits, entityIdentifier.IdentifierBits, CollisionType.OnTriggerStay));
        }
    }
}