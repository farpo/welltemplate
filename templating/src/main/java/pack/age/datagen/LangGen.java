package ``MOD_GROUP``.datagen;

import net.fabricmc.fabric.api.datagen.v1.FabricDataOutput;
import net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider;
import net.minecraft.block.Block;
import net.minecraft.entity.EntityType;
import net.minecraft.item.SpawnEggItem;
import net.minecraft.registry.RegistryWrapper;

import java.util.concurrent.CompletableFuture;

public class LangGen extends FabricLanguageProvider {
    protected LangGen(FabricDataOutput dataOutput, CompletableFuture<RegistryWrapper.WrapperLookup> registryLookup) {
        super(dataOutput, registryLookup);
    }

    private void generateBlockTranslations(RegistryWrapper.WrapperLookup wrapperLookup, TranslationBuilder tb){
    }
    private void generateBlockAndItem(TranslationBuilder translationBuilder, Block block, String translation){

    }
    private void generateItemTranslations(RegistryWrapper.WrapperLookup wrapperLookup, TranslationBuilder translationBuilder){

    }
    private void generateEntityTranslations(RegistryWrapper.WrapperLookup wrapperLookup, TranslationBuilder translationBuilder){

    }


    @Override
    public void generateTranslations(RegistryWrapper.WrapperLookup wrapperLookup, TranslationBuilder translationBuilder) {
        generateBlockTranslations(wrapperLookup, translationBuilder);
        generateItemTranslations(wrapperLookup, translationBuilder);
        generateEntityTranslations(wrapperLookup, translationBuilder);
    }
    private void generateEntityTranslationWithSpawnEgg(RegistryWrapper.WrapperLookup wrapperLookup, TranslationBuilder translationBuilder, EntityType<?> entityType, String name){
        translationBuilder.add(entityType, name);
        if(SpawnEggItem.forEntity(entityType) instanceof SpawnEggItem item){
            translationBuilder.add(item, name + " Spawn Egg");
        }
    }
}
