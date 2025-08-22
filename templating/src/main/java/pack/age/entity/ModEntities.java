package ``MOD_GROUP``.entity;

import ``MOD_GROUP``.``ENTRYPOINT_NAME``;
import net.fabricmc.fabric.api.itemgroup.v1.ItemGroupEvents;
import net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry;
import net.minecraft.entity.Entity;
import net.minecraft.entity.EntityType;
import net.minecraft.entity.LivingEntity;
import net.minecraft.entity.attribute.DefaultAttributeContainer;
import net.minecraft.entity.mob.MobEntity;
import net.minecraft.item.Item;
import net.minecraft.item.ItemGroups;
import net.minecraft.item.SpawnEggItem;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;
import net.minecraft.registry.RegistryKeys;

import java.util.function.Consumer;
import java.util.function.Supplier;

public class ``ENTITY_PREFIX``Entities {

    private static <T extends MobEntity> EntityType<T> createWithSpawnEgg(String name,
                                                                          EntityType.Builder<T> entityType,
                                                                          Supplier<DefaultAttributeContainer> containerSupplier){
        RegistryKey<Item> eggKey = RegistryKey.of(RegistryKeys.ITEM, ``ENTRYPOINT_NAME``.id(name + "_spawn_egg"));
        return createLiving(name, entityType, containerSupplier, tEntityType -> {
            SpawnEggItem eggItem = new SpawnEggItem(tEntityType, new Item.Settings().registryKey(eggKey));
            ItemGroupEvents.modifyEntriesEvent(ItemGroups.SPAWN_EGGS).register(fabricItemGroupEntries -> fabricItemGroupEntries.add(eggItem));
            Registry.register(Registries.ITEM, eggKey, eggItem);
        });
    }
    private static <T extends LivingEntity> EntityType<T> createLiving(String name, EntityType.Builder<T> builder,
                                                                 Supplier<DefaultAttributeContainer> containerSupplier, Consumer<EntityType<T>> onBuilt){
        return create(name, builder, onBuilt.andThen(tEntityType -> FabricDefaultAttributeRegistry.register(tEntityType, containerSupplier.get())));
    }
    private static <T extends LivingEntity> EntityType<T> createLiving(String name, EntityType.Builder<T> builder,
                                                                       Supplier<DefaultAttributeContainer> containerSupplier){
        return create(name, builder, tEntityType -> FabricDefaultAttributeRegistry.register(tEntityType, containerSupplier.get()));
    }
    public static <T extends Entity> EntityType<T> create(String name, EntityType.Builder<T> builder, Consumer<EntityType<T>> onBuilt){
        RegistryKey<EntityType<?>> key = RegistryKey.of(RegistryKeys.ENTITY_TYPE, ``ENTRYPOINT_NAME``.id(name));
        EntityType<T> type = builder.build(key);
        onBuilt.accept(type);
        return Registry.register(Registries.ENTITY_TYPE, key, type);
    }
    public static <T extends Entity> EntityType<T> create(String name, EntityType.Builder<T> builder){
        return create(name, builder, tEntityType -> {});
    }
    public static void init(){}
    public static class Tags{

    }
}
